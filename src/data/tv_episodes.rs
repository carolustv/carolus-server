// Copyright (c) 2017 Simon Dickson
// 
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use data::models::{TvEpisode, NewTvEpisode};
use data::schema;
use diesel::prelude::*;
use chrono::prelude::*;
use diesel;

pub fn create_tv_episode<'a>(conn: &SqliteConnection, tv_episode_series_id: i32, tv_episode_episode_number: i32, tv_episode_file_path: &'a str) -> TvEpisode {
    use data::schema::tv_episodes::dsl::*;

    let new_tv_episode = NewTvEpisode {
        tv_series_id: tv_episode_series_id,
        episode_number: tv_episode_episode_number,
        file_path: tv_episode_file_path,
        created: Utc::now().naive_utc(),
    };

    let tv_episode_id : Result<i32, _> =
        tv_episodes.filter(file_path.eq(tv_episode_file_path))
            .select(id)
            .first(conn);

    let tv_episode_id =
        match tv_episode_id {
            Ok(tv_episode_id) => tv_episode_id as usize,
            Err(_) => {
                diesel::insert_into(schema::tv_episodes::table)
                    .values(&new_tv_episode)
                    .execute(conn)
                    .expect("Error saving new tv_episode")
            }
        };
    get_tv_episode(conn, tv_episode_id as i64)
}

pub fn page_tv_episodes(conn: &SqliteConnection, page: i64, count: i64) -> Vec<TvEpisode> {
    use data::schema::tv_episodes::dsl::*;

    tv_episodes.offset(page * count)
        .limit(count)
        .load::<TvEpisode>(conn)
        .expect("Error loading tv_episodes")
}

pub fn get_tv_episode(conn: &SqliteConnection, tv_episode_id: i64) -> TvEpisode {
    use data::schema::tv_episodes::dsl::*;

    tv_episodes.find(tv_episode_id as i32)
        .first::<TvEpisode>(conn)
        .expect("Error loading tv_episode")
}
