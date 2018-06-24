table! {
    movies (id) {
        id -> Integer,
        title -> Text,
        file_path -> Text,
        background_image -> Nullable<Text>,
        card_image -> Nullable<Text>,
        created -> Timestamp,
        updated -> Timestamp,
    }
}

table! {
    tv_episodes (id) {
        id -> Integer,
        tv_series_id -> Integer,
        episode_number -> Integer,
        file_path -> Text,
        created -> Timestamp,
    }
}

table! {
    tv_series (id) {
        id -> Integer,
        tv_show_id -> Integer,
        series_number -> Integer,
        created -> Timestamp,
    }
}

table! {
    tv_shows (id) {
        id -> Integer,
        title -> Text,
        background_image -> Nullable<Text>,
        card_image -> Nullable<Text>,
        created -> Timestamp,
        updated -> Timestamp,
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
