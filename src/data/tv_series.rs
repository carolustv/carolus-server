// Copyright (c) 2017 Simon Dickson
// 
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use rusqlite::Connection;
use time::Timespec;

use data::models::TvSeries;

pub fn page_tv_series(conn: &Connection, page: i64, count: i64) -> Vec<TvSeries> {
    vec![]
}

pub fn get_tv_series(conn: &Connection, tv_series_id: i64) -> TvSeries {
    TvSeries {
        id: 1,
        series_number: 1,
        tv_show_id: 1,
        created: Timespec::new(0, 0),
        updated: Timespec::new(0, 0)
    }
}
