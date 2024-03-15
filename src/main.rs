mod MongoManager;
mod MongoConnectionSettings;
mod MongoCollection;
use actix_web::{get,  App, HttpResponse, HttpServer, Responder};
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
    let mongo = MongoManager::MongoManager::new(
        MongoConnectionSettings::MongoConnectionSettings::new(
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
    let collection = mongo.get_collection::<Restaurant>();

    let result =collection.insert(doc).await;
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
