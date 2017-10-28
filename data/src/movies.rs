// Copyright (c) 2017 Simon Dickson
// 
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use models::{Movie, NewMovie};
use schema::movies;
use diesel::prelude::*;
use chrono::prelude::*;
use diesel;

pub fn create_movie<'a>(conn: &SqliteConnection, title: &'a str, file_path: &'a str) -> Movie {
    let new_movie = NewMovie {
        title: title,
        file_path: file_path,
        created_date: Utc::now().naive_utc(),
    };

    let id =
        diesel::insert_or_replace(&new_movie)
            .into(movies::table)
            .execute(conn)
            .expect("Error saving new movie");

    get_movie(conn, id as i64)
}

pub fn page_movies(conn: &SqliteConnection, page: i64, count: i64) -> Vec<Movie> {
    use schema::movies::dsl::*;

    movies.offset(page * count)
        .limit(count)
        .load::<Movie>(conn)
        .expect("Error loading movies")
}

pub fn get_movie(conn: &SqliteConnection, id: i64) -> Movie {
    use schema::movies::dsl::*;

    movies.find(id)
        .first::<Movie>(conn)
        .expect("Error loading movie")
}
