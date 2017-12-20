// Copyright (c) 2017 Simon Dickson
// 
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use data::models::{Movie, NewMovie};
use data::schema;
use diesel::prelude::*;
use chrono::prelude::*;
use diesel;

pub fn create_movie<'a>(conn: &SqliteConnection, movie_title: &'a str, formatted_title: &'a str, movie_file_path: &'a str) -> Result<Movie, diesel::result::Error> {
    use data::schema::movies::dsl::*;

    let new_movie = NewMovie {
        title: movie_title,
        formatted_title: movie_title,
        file_path: movie_file_path,
        created_date: Utc::now().naive_utc(),
    };

    match get_movie(conn, new_movie.formatted_title) {
        Ok(movie) => Ok(movie),
        Err(_) => {
            diesel::insert(&new_movie)
                .into(schema::movies::table)
                .execute(conn)?;
            get_movie(&conn, &new_movie.formatted_title)
        }
    }
}

pub fn page_movies(conn: &SqliteConnection, page: i64, count: i64) -> Vec<Movie> {
    use data::schema::movies::dsl::*;

    movies.offset(page * count)
        .limit(count)
        .load::<Movie>(conn)
        .expect("Error loading movies")
}

pub fn get_movie(conn: &SqliteConnection, movie_formatted_title: &str) -> Result<Movie, diesel::result::Error> {
    use data::schema::movies::dsl::*;

    movies.filter(formatted_title.eq(movie_formatted_title))
        .first::<Movie>(conn)
}
