table! {
    movies (id) {
        id -> Integer,
        title -> Text,
        file_path -> Text,
        created_date -> Timestamp,
    }
}

table! {
    tv_episodes (id) {
        id -> Integer,
        tv_series_id -> Integer,
        episode_number -> Integer,
        file_path -> Text,
        created_date -> Timestamp,
    }
}

table! {
    tv_series (id) {
        id -> Integer,
        tv_show_id -> Integer,
        series_number -> Integer,
        created_date -> Timestamp,
    }
}

table! {
    tv_shows (id) {
        id -> Integer,
        title -> Text,
        created_date -> Timestamp,
    }
}

joinable!(tv_episodes -> tv_series (tv_series_id));
joinable!(tv_series -> tv_shows (tv_show_id));

allow_tables_to_appear_in_same_query!(
    movies,
    tv_episodes,
    tv_series,
    tv_shows,
);
