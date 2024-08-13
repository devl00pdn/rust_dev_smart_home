use futures::StreamExt;
use mongodb::{Client, Collection};
use mongodb::bson::{doc, ser};
use mongodb::bson::oid::ObjectId;
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

impl SmartSocketData {
    pub fn new(name: String, description: String) -> Self { Self { name, description, power_consumption_wt: 0.0, state: false } }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SmartThermometerData {
    name: String,
    description: String,
    current_temp_deg: f32,
}

impl SmartThermometerData {
    pub fn new(name: String, description: String) -> Self { Self { name, description, current_temp_deg: 0.0 } }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct NewDevice {
    name: String,
    description: String,
    device_type: String,
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
        let cl = Client::with_uri_str(connection_str).await.unwrap();
        // cl.database("rooms_db").collection::<RoomData>("rooms").create_index(index, None).await.unwrap();
        cl.database("rooms_db").collection::<RoomData>("rooms");
        Self(cl)
    }

    pub async fn create_room(&self, data: RoomData) -> CustomResult<RoomData> {
        let collection = self.0.database("rooms_db").collection("rooms");
        let query = doc! { "name": &data.name };
        let room = collection.find_one(query, None).await?;
        if room.is_some() {
            return Err(CustomError::InternalError(format!("rooms with name: {} already exist", data.name.as_str())));
        }
        let inserted = collection.insert_one(data, None).await?;
        let id = inserted.inserted_id;
        let query = doc! { "_id": &id };
        let room = collection.find_one(query, None).await?;
        room.ok_or_else(|| CustomError::NotFound(format!("rooms with id: {}", id)))
    }

    pub async fn read_room(&self, id: ObjectId) -> CustomResult<RoomData> {
        let collection = self.0.database("rooms_db").collection("rooms");
        let query = doc! { "_id": &id };
        let room = collection.find_one(query, None).await?;
        room.ok_or_else(|| CustomError::NotFound(format!("room with id: {}", id)))
    }

    pub async fn delete_room(&self, id: ObjectId) -> CustomResult<()> {
        let collection: Collection<RoomData> = self.0.database("rooms_db").collection("rooms");
        let query = doc! { "_id": &id };
        collection.find_one_and_delete(query, None).await?;
        Ok(())
    }

    pub async fn read_rooms(&self) -> CustomResult<Vec<RoomData>> {
        let collection = self.0.database("rooms_db").collection("rooms");
        let query = doc! {};
        let mut rooms = collection.find(query, None).await?;
        let mut rooms_vec = Vec::new();
        while let Some(room) = rooms.next().await {
            rooms_vec.push(room?);
        }
        Ok(rooms_vec)
    }

    pub async fn create_device(&self, id: ObjectId, data: &NewDevice) -> CustomResult<Device> {
        let collection: Collection<RoomData> = self.0.database("rooms_db").collection("rooms");

        let device = match data.device_type.as_str() {
            "socket" => { Some(Device::Socket(SmartSocketData::new(data.name.clone(), data.description.clone()))) }
            "thermometer" => { Some(Device::Thermometer(SmartThermometerData::new(data.name.clone(), data.description.clone()))) }
            _ => { None }
        }.ok_or(CustomError::DeviceTypeError(data.device_type.clone()))?;

        if self.read_devices(id).await?.iter().any(|t| match t {
            Device::Socket(d) => { d.name == data.name }
            Device::Thermometer(d) => { d.name == data.name }
        }) {
            return Err(CustomError::InternalError(format!("device with name: {} already exist", data.name.as_str())));
        }
        let query = doc! { "_id": &id };
        let update = doc! { "$push": {"devices": ser::to_bson(&device)? } };
        collection.update_one(query, update, None).await?;
        self.read_device(id, &data.name).await
    }

    pub async fn read_devices(&self, id: ObjectId) -> CustomResult<Vec<Device>> {
        self.read_room(id).await.map(|b| b.devices)
    }

    pub async fn read_device(&self, id: ObjectId, name: &str) -> CustomResult<Device> {
        let room = self.read_room(id).await?;
        let device = room.devices.into_iter().find(|t| match t {
            Device::Socket(d) => { d.name == name }
            Device::Thermometer(d) => { d.name == name }
        });
        device.ok_or_else(|| CustomError::NotFound(format!("device with name: {}", name)))
    }

    pub async fn delete_device(&self, id: ObjectId, name: &str) -> CustomResult<Device> {
        let device = self.read_device(id, name).await?;
        let collection: Collection<RoomData> = self.0.database("rooms_db").collection("rooms");
        let filter = doc! {
            "_id": id,
            "devices": {
                "$elemMatch": { "Thermometer.name": name }
            }
        };
        let update = doc! {
            "$pull": {
                "devices": { "Thermometer.name": name }
            }
        };
        collection.update_one(filter, update, None).await?;
        Ok(device)
    }
}

