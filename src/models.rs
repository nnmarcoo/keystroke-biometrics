use crate::schema::{user, metrics, pairs};

#[derive(Queryable, Insertable, Identifiable)]
#[table_name = "user"]
pub struct User {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable)]
#[table_name = "user"]
pub struct NewUser<'a> {
    pub name: &'a str,
}

#[derive(Queryable, Insertable)]
#[table_name = "metrics"]
pub struct Metric {
    pub id: i32,
    pub WPM: f32,
    pub CPE: f32,
}

#[derive(Queryable, Insertable)]
#[table_name = "pairs"]
pub struct Pair {
    pub id: i32,
    pub pair: String,
    pub interval: f32,
}