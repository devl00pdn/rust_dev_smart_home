use std::env;
use std::error::Error;
use std::str::FromStr;
use std::sync::Arc;

use actix_web::{App, HttpResponse, HttpServer, web};
use actix_web::dev::Service;
use actix_web::web::{Data, Path};
use log::LevelFilter;
use mongodb::bson::oid::ObjectId;

use mongo::MongoHouse;

use crate::count::CountersTransform;
use crate::error::CustomResult;
use crate::mongo::{NewDevice, RoomData};

mod error;
mod mongo;
mod count;


#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv()?;
    env_logger::builder()
        .filter_level(LevelFilter::Debug)
        .init();

    let mongo = MongoHouse::new(&env::var("MONGO_CONNECTION")?).await;
    let boards_data = Arc::new(mongo);
    let counters = CountersTransform::default();


    HttpServer::new(move || {
        App::new()
            .wrap(counters.clone())
            .wrap_fn(|req, srv| {
                let addr = req.peer_addr();
                log::info!("From middleware fn: Hello {addr:?}");
                srv.call(req)
            })
            .app_data(Data::new(boards_data.clone()))
            .service(create_room)
            .service(read_rooms)
            .service(create_device)
            .default_service(web::to(default_response))
    })
        .bind("0.0.0.0:8080")?
        .run()
        .await?;

    Ok(())
}

async fn default_response() -> CustomResult<HttpResponse> {
    Ok(HttpResponse::Ok().body("Go to '/board'"))
}

#[actix_web::post("/room")]
async fn create_room(
    rooms_data: web::Json<RoomData>,
    rooms: web::Data<Arc<MongoHouse>>,
) -> CustomResult<HttpResponse> {
    let data = rooms_data.into_inner();
    let created = rooms.create_room(data).await?;
    Ok(HttpResponse::Ok().json(created))
}

#[actix_web::get("/room")]
async fn read_rooms(rooms: web::Data<Arc<MongoHouse>>) -> CustomResult<HttpResponse> {
    let boards = rooms.read_rooms().await?;
    Ok(HttpResponse::Ok().json(boards))
}


#[actix_web::post("/room/{id}/device")]
async fn create_device(
    path: Path<String>,
    device_data: web::Json<NewDevice>,
    rooms: web::Data<Arc<MongoHouse>>,
) -> CustomResult<HttpResponse> {
    let data = device_data.into_inner();
    let id = ObjectId::from_str(&path.into_inner())?;
    let created = rooms.create_device(id, &data).await?;
    Ok(HttpResponse::Ok().json(created))
}
