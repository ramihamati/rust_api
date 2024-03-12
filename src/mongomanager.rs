use std::fmt::Error;
use actix_web::cookie::time::macros::date;
use mongodb::{
    bson::{Document, doc},
    Client,
    Collection
};
use mongodb::bson::Bson;
use mongodb::bson::oid::ObjectId;
use serde::Serialize;

#[derive(Clone)]
pub(crate) struct MongoConnectionSettings{
    connection_string : String,
    database: String,
    table_name: String
}

impl MongoConnectionSettings{
     pub(crate) fn new(connection_string: String, database: String, table_name: String) -> MongoConnectionSettings{
        return MongoConnectionSettings{
            connection_string,
            database,
            table_name
        }
    }
}
pub(crate) struct MongoManager<T : Serialize>{
    settings : MongoConnectionSettings,
    client: Collection<T>
}

impl<T> MongoManager<T> where T : Serialize {
    pub(crate) async fn new(settings: MongoConnectionSettings) -> MongoManager<T>{
        let cSettings = settings.clone();

        let clientResult = Client::with_uri_str(
            cSettings.connection_string).await;

        match clientResult {
            Ok(client) => {
                let cSettings = settings.clone();

                let database = client.database(cSettings.database.as_str());

                let collection: Collection<T> = database.collection(cSettings.table_name.as_str());
                return MongoManager{
                    client: collection,
                    settings: settings.clone()
                }
            }
            Err(_) => {
                panic!("Failed to create the client")
            }
        }

    }

    pub(crate) async fn insert(&self, document: T) -> Result<ObjectId, Error>{
        let result = self
            .client
            .insert_one(document, None)
            .await;

        match result {
            Ok(value) => {
                let objectId = value.inserted_id.as_object_id();
                match objectId {
                    None => {
                        // TODO: how to return a string in the error
                        return Err(Error);
                    }
                    Some(value) => {
                        return Ok(value);
                    }
                }
            }
            Err(err ) => {
                // how to forward the error
                return Result::Err(Error);
            }
        }
    }
}