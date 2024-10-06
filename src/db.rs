use diesel::{Connection, MysqlConnection};

use crate::constants;

pub fn establish_connection() -> MysqlConnection {
    MysqlConnection::establish(&constants::DATABASE_URL)
        .unwrap_or_else(|_| panic!("Error connecting to {}", constants::DATABASE_URL))
}
