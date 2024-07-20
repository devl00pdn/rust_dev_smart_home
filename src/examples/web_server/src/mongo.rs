use futures::StreamExt;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::Client;
use serde::{Deserialize, Serialize};

use crate::error::{CustomError, CustomResult};

#[derive(Clone, Serialize, Deserialize)]
pub struct HouseData {
    name: String,
    rooms: Vec<RoomData>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct RoomData {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    name: String,
    devices: Vec<Device>,
}


#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SmartSocketData {
    name: String,
    description: String,
    power_consumption_wt: f32,
    state: bool,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SmartThermometerData {
    description: String,
    current_temp_deg: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Device {
    Socket(SmartSocketData),
    Thermometer(SmartThermometerData),
}


#[derive(Clone)]
pub struct MongoHouse(Client);

impl MongoHouse {
    pub async fn new(connection_str: &str) -> Self {
        Self(Client::with_uri_str(connection_str).await.unwrap())
    }

    pub async fn create_room(&self, data: RoomData) -> CustomResult<RoomData> {
        let collection = self.0.database("rooms_db").collection("rooms");
        let inserted = collection.insert_one(data, None).await?;
        let id = inserted.inserted_id;
        let query = doc! { "_id": &id };
        let board = collection.find_one(query, None).await?;
        board.ok_or_else(|| CustomError::NotFound(format!("rooms with id: {}", id)))
    }


    pub async fn read_rooms(&self) -> CustomResult<Vec<RoomData>> {
        let collection = self.0.database("rooms_db").collection("rooms");
        let query = doc! {};
        let mut rooms = collection.find(query, None).await?;

        let mut rooms_vec = Vec::new();
        while let Some(board) = rooms.next().await {
            rooms_vec.push(board?);
        }
        Ok(rooms_vec)
    }
}

