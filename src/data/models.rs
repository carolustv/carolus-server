// Copyright (c) 2017 Simon Dickson
// 
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use time::Timespec;

#[derive(PartialEq, Debug)]
pub struct Movie {
    pub id: i32,
    pub title: String,
    pub file_path: String,
    pub background_image: Option<String>,
    pub card_image: Option<String>,
    pub created: Timespec,
    pub updated: Timespec
}

#[derive(PartialEq, Debug)]
pub struct TvShow {
    pub id: i32,
    pub title: String,
    pub background_image: Option<String>,
    pub card_image: Option<String>,
    pub created: Timespec,
    pub updated: Timespec
}


#[derive(PartialEq, Debug)]
pub struct TvSeries {
    pub id: i32,
    pub series_number: i32,
    pub tv_show_id: i32,
    pub created: Timespec,
    pub updated: Timespec
}

#[derive(PartialEq, Debug)]
pub struct TvEpisode {
    pub id: i32,
    pub tv_series_id: i32,
    pub episode_number: i32,
    pub file_path: String,
    pub created: Timespec,
    pub updated: Timespec
}
