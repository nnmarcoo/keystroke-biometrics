use diesel::{Connection, MysqlConnection};
use crate::constants;

pub fn establish_connection() -> Option<MysqlConnection> {
    MysqlConnection::establish(&constants::DATABASE_URL).ok()
}
