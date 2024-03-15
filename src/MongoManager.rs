use mongodb::{Client, Collection, Database};
use serde::Serialize;
use crate::MongoCollection::MongoCollection;
use crate::MongoConnectionSettings::MongoConnectionSettings;

pub(crate) struct MongoManager{
    settings : MongoConnectionSettings,
    database: Database
}
impl MongoManager {
    pub(crate) async fn new(settings: MongoConnectionSettings) -> MongoManager{
        let c_settings = settings.clone();

        let client_result = Client::with_uri_str(
            c_settings.connection_string).await;

        match client_result {
            Ok(client) => {
                let database = client.database(c_settings.database.as_str());
                return MongoManager{
                    database,
                    settings: settings.clone()
                }
            }
            Err(_) => {
                panic!("Failed to create the client")
            }
        }
    }

    pub (crate) fn get_collection<T>(&self) -> MongoCollection<T> where T : Serialize
    {
        let collection: Collection<T> = self.database.collection(
            self.settings.table_name.as_str());

        return MongoCollection::new(collection);
    }
}