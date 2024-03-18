use serde::Serialize;
use mongodb::Collection;
use mongodb::bson::oid::ObjectId;
use std::fmt::{Display, Formatter, Write};
use mongodb::bson::doc;
use crate::CustomError::CustomError;

pub(crate) struct MongoCollection<T : Serialize>
{
    client: Collection<T>
}

pub(crate) struct MongoError {
    Error : String
}

impl CustomError for MongoError{
    fn get_error(&self) -> String {
        return self.Error.clone();
    }
}

impl Display for MongoError{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        return f.write_str(self.Error.as_str())
    }
}
impl<T> MongoCollection<T> where T : Serialize {
    pub(crate) fn new(client: Collection<T>) -> MongoCollection<T>{
        return  MongoCollection{
            client
        }
    }

    pub(crate) async fn find_by_id(&self, id: ObjectId) -> Result<T, MongoError>{

        let result = self.client
            .find(doc! {
                "_id" : id
            }, None).await;

        match result{
            Ok(cursor) =>{
                match cursor.deserialize_current(){
                    Ok(document) =>{
                        return document;
                    }
                    Err(err) => Err(MongoError{Error : err.to_string()})
                }
            },
            Err(err) => Err(MongoError{Error : err.to_string()})
        }
    }

    pub(crate) async fn insert(&self, document: T) -> Result<ObjectId, MongoError> {
        let result = self
            .client
            .insert_one(document, None)
            .await;

        match result {
            Ok(value) => {
                let objectId = value.inserted_id.as_object_id();
                match objectId {
                    None => Err(MongoError{ Error : "Failed to retrieve the object id".to_string()}),
                    Some(value) => {
                        return Ok(value);
                    }
                }
            }
            Err(err) => Err(MongoError{ Error : err.to_string()})
        }
    }
}
