// Copyright (c) 2017 Simon Dickson
// 
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use rusqlite::Connection;
use time::Timespec;

use data::models::{TvEpisode, TvShow, TvSeries};

pub fn page_tv_shows(conn: &Connection, page: i64, count: i64) -> Vec<(TvShow, Vec<(TvSeries, Vec<TvEpisode>)>)> {
    vec![]
}

pub fn get_tv_show(conn: &Connection, tv_show_id: i64) -> TvShow {
    TvShow {
        id: 1,
        title: "".to_owned(),
        background_image: Some("".to_owned()),
        card_image: Some("".to_owned()),
        created: Timespec::new(0, 0),
        updated: Timespec::new(0, 0)
    }
}
