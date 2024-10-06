// @generated automatically by Diesel CLI.

diesel::table! {
    metrics (id) {
        id -> Integer,
        WPM -> Float,
        CPE -> Float,
    }
}

diesel::table! {
    pairs (id, pair) {
        id -> Integer,
        #[max_length = 2]
        pair -> Char,
        interval -> Float,
    }
}

diesel::table! {
    user (id) {
        id -> Integer,
        #[max_length = 255]
        name -> Varchar,
    }
}

diesel::joinable!(metrics -> user (id));
diesel::joinable!(pairs -> user (id));

diesel::allow_tables_to_appear_in_same_query!(
    metrics,
    pairs,
    user,
);
