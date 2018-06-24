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

pub fn create_movie<'a>(conn: &SqliteConnection, movie_title: &'a str, movie_file_path: &'a str) -> Movie {
    use data::schema::movies::dsl::*;

    let new_movie = NewMovie {
        title: movie_title,
        file_path: movie_file_path,
        created: Utc::now().naive_utc(),
        updated: Utc::now().naive_utc(),
    };

    let movie_id : Result<i32, _> =
        movies.filter(file_path.eq(movie_file_path))
            .select(id)
            .first(conn);

    let movie_id =
        match movie_id {
            Ok(movie_id) => movie_id as usize,
            Err(_) => {
                diesel::insert_into(schema::movies::table)
                    .values(&new_movie)
                    .execute(conn)
                    .expect("Error saving new movie")
            }
        };
        
    get_movie(conn, movie_id as i64)
}

pub fn update_movie_metadata(conn: &SqliteConnection, movie_id: i32, movie_background_image: &str, movie_card_image: &str) {
    use data::schema::movies::dsl::*;

    diesel::update(schema::movies::table)
        .filter(id.eq(movie_id))
        .set((
            background_image.eq(movie_background_image),
            card_image.eq(movie_card_image),
        ));
}

pub fn page_movies(conn: &SqliteConnection, page: i64, count: i64) -> Vec<Movie> {
    use data::schema::movies::dsl::*;

    movies.offset(page * count)
        .limit(count)
        .load::<Movie>(conn)
        .expect("Error loading movies")
}

pub fn get_all_movies(conn: &SqliteConnection) -> Vec<Movie> {
    use data::schema::movies::dsl::*;

    movies.load::<Movie>(conn)
        .expect("Error loading movies")
}

pub fn get_movie(conn: &SqliteConnection, movie_id: i64) -> Movie {
    use data::schema::movies::dsl::*;

    movies.find(movie_id as i32)
        .first::<Movie>(conn)
        .expect("Error loading movie")
}
