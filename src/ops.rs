use crate::data::Data;
use crate::db::establish_connection;
use crate::models::{Metric, NewUser, Pair};
use diesel::prelude::*;

pub fn create_user(user_name: &str) -> QueryResult<i32> {
    use crate::schema::user::dsl::*;
    let mut conn = establish_connection().unwrap();

    let new_user = NewUser { name: user_name };

    diesel::insert_into(user)
        .values(&new_user)
        .execute(&mut conn)?;

    // Fetch the inserted user to get the generated ID
    user.order(id.desc()) // Get the most recent user inserted
        .select(id) // Only select the ID
        .first(&mut conn) // Retrieve the first (most recent) entry
}

pub fn clear_users() -> QueryResult<usize> {
    use crate::schema::user::dsl::*;
    let mut conn = establish_connection().unwrap();

    diesel::delete(user).execute(&mut conn)
}

pub fn insert_metrics(user_id: i32, user_wpm: f32, user_cpe: f32) {
    use crate::schema::metrics::dsl::*;
    let mut conn = establish_connection().unwrap();
    let new_metric = Metric {
        id: user_id,
        WPM: user_wpm,
        CPE: user_cpe,
    };

    let _ = diesel::insert_into(metrics)
        .values(&new_metric)
        .execute(&mut conn);
}

pub fn clear_metrics() -> QueryResult<usize> {
    use crate::schema::metrics::dsl::*;
    let mut conn = establish_connection().unwrap();

    diesel::delete(metrics).execute(&mut conn)
}

pub fn insert_pairs(user_id: i32, type_data: &Data) {
    use crate::schema::pairs::dsl::*;
    let mut conn = establish_connection().unwrap();

    let data = type_data.clean_pairs(2.);

    for (k, v) in data.iter() {
        let key_pair = format!("{}{}", k.0, k.1);

        let new_pair = Pair {
            id: user_id,
            pair: key_pair,
            interval: v.as_secs_f32() * 1000.,
        };

        let _ = diesel::insert_into(pairs)
            .values(&new_pair)
            .execute(&mut conn);
    }
}

pub fn clear_pairs() -> QueryResult<usize> {
    use crate::schema::pairs::dsl::*;
    let mut conn = establish_connection().unwrap();

    diesel::delete(pairs).execute(&mut conn)
}
