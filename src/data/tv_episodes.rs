// Copyright (c) 2017 Simon Dickson
// 
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use failure::Error;
use rusqlite::Connection;
use time::{Timespec, get_time};

use data::models::TvEpisode;

pub fn create_tv_episode<'a>(conn: &Connection, show_title: &'a str, series_number: i32, episode_number: i32, file_path: &'a str) -> Result<(), Error> {
    let now = &get_time();

    conn.execute("INSERT INTO tv_shows (title, created, updated)
                  VALUES (?1, ?2, ?3)",
                 &[&show_title, now, now])?;

    let show_id = conn.last_insert_rowid();

    conn.execute("INSERT INTO tv_series (tv_show_id, series_number, created, updated)
                  VALUES (?1, ?2, ?3, ?4)",
                 &[&show_id, &series_number, now, now])?;

    let series_id = conn.last_insert_rowid();

    conn.execute("INSERT INTO tv_episodes (tv_series_id, episode_number, file_path, created, updated)
                  VALUES (?1, ?2, ?3, ?4, ?5)",
                 &[&series_id, &episode_number, &file_path, now, now])?;

    Ok(())
}

pub fn page_tv_episodes(conn: &Connection, page: i64, count: i64) -> Vec<TvEpisode> {
    vec![]
}

pub fn get_tv_episode(conn: &Connection, tv_episode_id: i64) -> TvEpisode {
    TvEpisode {
        id: 1,
        tv_series_id: 1,
        file_path: "".to_owned(),
        episode_number: 1,
        created: Timespec::new(0, 0),
        updated: Timespec::new(0, 0)
    }
}
