// Copyright (c) 2017 Simon Dickson
// 
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use data::models::{TvShow, NewTvShow};
use data::schema;
use diesel::prelude::*;
use chrono::prelude::*;
use diesel;

pub fn create_tv_show<'a>(conn: &SqliteConnection, tv_show_title: &'a str) -> TvShow {
    use data::schema::tv_shows::dsl::*;

    let new_tv_show = NewTvShow {
        title: tv_show_title,
        created_date: Utc::now().naive_utc(),
    };

    let tv_show_id : Result<i32, _> =
        tv_shows.filter(title.eq(tv_show_title))
            .select(id)
            .first(conn);

    let tv_show_id =
        match tv_show_id {
            Ok(tv_show_id) => tv_show_id as usize,
            Err(_) => {
                diesel::insert_into(schema::tv_shows::table)
                    .values(&new_tv_show)
                    .execute(conn)
                    .expect("Error saving new tv_show")
            }
        };
    get_tv_show(conn, tv_show_id as i64)
}

pub fn page_tv_shows(conn: &SqliteConnection, page: i64, count: i64) -> Vec<TvShow> {
    use data::schema::tv_shows::dsl::*;

    tv_shows.offset(page * count)
        .limit(count)
        .load::<TvShow>(conn)
        .expect("Error loading tv_shows")
}

pub fn get_tv_show(conn: &SqliteConnection, tv_show_id: i64) -> TvShow {
    use data::schema::tv_shows::dsl::*;

    tv_shows.find(tv_show_id as i32)
        .first::<TvShow>(conn)
        .expect("Error loading tv_show")
}
