mod mongomanager;

use std::fmt::Error;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use mongodb::bson::Bson;
use crate::mongomanager::{MongoConnectionSettings};
use crate::mongomanager::MongoManager;
use serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize, Debug)]
struct Restaurant {
    borough: String,
    cuisine: String,
    name: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

#[get("/")]
async fn hello() -> impl Responder {
    let mongo = MongoManager::<Restaurant>::new(
        MongoConnectionSettings::new(
            String::from("mongodb://localhost:27017"),
            String::from("database"),
            String::from("table")
        )
    ).await;
    let doc = Restaurant {
        name: "Sea Stone Tavern".to_string(),
        cuisine: "Greek".to_string(),
        borough: "Queens".to_string(),
    };

    let result = mongo.insert(doc).await;
    match result {
        Ok(id) => {
            println!("{}", id)
        }
        Err(err) => {
            println!("{}", err)
        }
    }
    HttpResponse::Ok().body("Hello world!")
}