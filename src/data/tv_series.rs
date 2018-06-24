// Copyright (c) 2017 Simon Dickson
// 
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use data::models::{TvSeries, NewTvSeries};
use data::schema;
use diesel::prelude::*;
use chrono::prelude::*;
use diesel;

pub fn create_tv_series<'a>(conn: &SqliteConnection, tv_series_show_id: i32, tv_series_number: i32) -> TvSeries {
    use data::schema::tv_series::dsl::*;

    let new_tv_series = NewTvSeries {
        tv_show_id: tv_series_show_id,
        series_number: tv_series_number,
        created: Utc::now().naive_utc(),
    };

    let tv_series_id : Result<i32, _> =
        tv_series.filter(tv_show_id.eq(tv_series_show_id))
            .select(id)
            .first(conn);

    let tv_series_id =
        match tv_series_id {
            Ok(tv_series_id) => tv_series_id as usize,
            Err(_) => {
                diesel::insert_into(schema::tv_series::table)
                    .values(&new_tv_series)
                    .execute(conn)
                    .expect("Error saving new tv_series")
            }
        };
    get_tv_series(conn, tv_series_id as i64)
}

pub fn page_tv_series(conn: &SqliteConnection, page: i64, count: i64) -> Vec<TvSeries> {
    use data::schema::tv_series::dsl::*;

    tv_series.offset(page * count)
        .limit(count)
        .load::<TvSeries>(conn)
        .expect("Error loading tv_series")
}

pub fn get_tv_series(conn: &SqliteConnection, tv_series_id: i64) -> TvSeries {
    use data::schema::tv_series::dsl::*;

    tv_series.find(tv_series_id as i32)
        .first::<TvSeries>(conn)
        .expect("Error loading tv_series")
}
