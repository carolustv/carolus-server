// Copyright (c) 2017 Simon Dickson
// 
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use failure::Error;
use rusqlite::Connection;
use time::{Timespec, get_time};

use data::models::Movie;

pub fn create_movie<'a>(conn: &Connection, title: &'a str, file_path: &'a str) -> Result<(), Error> {
    let now = &get_time();

    conn.execute("INSERT INTO movies (title, file_path, created, updated)
                  VALUES (?1, ?2, ?3, ?4)",
                 &[&title, &file_path, now, now])?;

    Ok(())
}

pub fn update_movie_metadata(_conn: &Connection, movie_id: i32, movie_background_image: &str, movie_card_image: &str) {
    
}

pub fn page_movies(conn: &Connection, page: i64, count: i64) -> Vec<Movie> {
    vec![]
}

pub fn get_all_movies(conn: &Connection) -> Vec<Movie> {
    vec![]
}

pub fn get_movie(conn: &Connection, movie_id: i64) -> Movie {
    Movie {
        id: 1,
        title: "".to_owned(),
        file_path: "".to_owned(),
        background_image: Some("".to_owned()),
        card_image: Some("".to_owned()),
        created: Timespec::new(0, 0),
        updated: Timespec::new(0, 0)
    }
}
