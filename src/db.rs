use diesel::{Connection, MysqlConnection};

pub fn establish_connection() -> MysqlConnection {

    let database_url = String::from("mysql://root@localhost/keys");
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
