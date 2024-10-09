use std::collections::HashMap;

use crate::data::Data;
use crate::db::establish_connection;
use crate::models::{Metric, NewUser, Pair, PairResult};
use diesel::prelude::*;
use diesel::sql_types::{Float, Text};

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

pub fn remove_user(user_id: i32) -> QueryResult<usize> {
    use crate::schema::metrics::dsl as metrics_dsl;
    use crate::schema::pairs::dsl as pairs_dsl;
    use crate::schema::user::dsl as user_dsl;

    let mut conn = establish_connection().unwrap();

    diesel::delete(pairs_dsl::pairs.filter(pairs_dsl::id.eq(user_id))).execute(&mut conn)?;

    diesel::delete(metrics_dsl::metrics.filter(metrics_dsl::id.eq(user_id))).execute(&mut conn)?;

    diesel::delete(user_dsl::user.filter(user_dsl::id.eq(user_id))).execute(&mut conn)
}

pub fn get_users() -> QueryResult<Vec<(i32, String)>> {
    use crate::schema::user::dsl::*;
    let mut conn = establish_connection().unwrap();

    let results = user.select((id, name)).load::<(i32, String)>(&mut conn)?;

    Ok(results)
}

pub fn get_metrics(user_id: i32) -> QueryResult<Option<(f32, f32)>> {
    use crate::schema::metrics::dsl::*;
    let mut conn = establish_connection().unwrap();

    metrics
        .filter(id.eq(user_id))
        .select((wpm, cpe)) 
        .first::<(f32, f32)>(&mut conn) 
        .optional()
}

pub fn insert_metrics(user_id: i32, user_wpm: f32, user_cpe: f32) {
    use crate::schema::metrics::dsl::*;
    let mut conn = establish_connection().unwrap();
    let new_metric = Metric {
        id: user_id,
        wpm: user_wpm,
        cpe: user_cpe,
    };

    let _ = diesel::insert_into(metrics)
        .values(&new_metric)
        .execute(&mut conn);
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

pub fn get_pairs(user_id: i32, pair_list: Vec<String>) -> QueryResult<Vec<f32>> {
    use crate::schema::pairs::dsl::*;
    let mut conn = establish_connection().unwrap();

    let interval_values = pairs
        .filter(id.eq(user_id))
        .filter(pair.eq_any(pair_list))
        .select(interval)
        .load::<f32>(&mut conn)?;

    Ok(interval_values)
}

pub fn match_pairs(type_data: &Data) -> HashMap<i32, usize> {
    let data = type_data.clean_pairs(2.);
    let mut conn = establish_connection().unwrap();

    let mut user_counts: HashMap<i32, usize> = HashMap::new();

    for (key, duration) in data.iter() {
        let key_pair = format!("{}{}", key.0, key.1);
        let input_interval = duration.as_secs_f32() * 1000.0;

        let results = diesel::sql_query(
            "SELECT u.id, u.name, p.pair, p.interval
             FROM pairs p, user u
             WHERE p.id = u.id
             AND p.pair = ?
             ORDER BY ABS(p.interval - ?)
             LIMIT 1",
        )
        .bind::<Text, _>(&key_pair)
        .bind::<Float, _>(input_interval)
        .load::<PairResult>(&mut conn)
        .expect("Error loading pairs");

        for result in results {
            *user_counts.entry(result.id).or_insert(0) += 1;
        }
    }

    user_counts
}

pub fn match_metrics(type_data: &Data) -> QueryResult<(i32, i32)> {
    use crate::schema::metrics::dsl::*;
    let mut conn = establish_connection().unwrap();

    let target_wpm = type_data.get_wpm_value();
    let target_cpe = type_data.get_cpe_value();

    let wpm_match = metrics
        .select(id)
        .order_by(diesel::dsl::sql::<Float>(&format!(
            "ABS(wpm - {})",
            target_wpm
        )))
        .first::<i32>(&mut conn)?;

    let cpe_match = metrics
        .select(id)
        .order_by(diesel::dsl::sql::<Float>(&format!(
            "ABS(cpe - {})",
            target_cpe
        )))
        .first::<i32>(&mut conn)?;

    Ok((wpm_match, cpe_match))
}
