use crate::schema::{metrics, pairs, user};
use diesel::sql_types::Integer;

#[derive(Queryable, Insertable, Identifiable)]
#[diesel(table_name = user)]
pub struct User {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable)]
#[diesel(table_name = user)]
pub struct NewUser<'a> {
    pub name: &'a str,
}

#[derive(Queryable, Insertable)]
#[diesel(table_name = metrics)]
pub struct Metric {
    pub id: i32,
    pub wpm: f32,
    pub cpe: f32,
}

#[derive(Queryable, Insertable)]
#[diesel(table_name = pairs)]
pub struct Pair {
    pub id: i32,
    pub pair: String,
    pub interval: f32,
}

#[derive(QueryableByName, Debug)]
pub struct PairResult {
    #[diesel(sql_type = Integer)]
    pub id: i32,
}
