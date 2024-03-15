#[derive(Clone)]
pub(crate) struct MongoConnectionSettings{
    pub(crate) connection_string : String,
    pub(crate) database: String,
    pub(crate) table_name: String
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
