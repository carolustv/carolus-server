// Copyright (c) 2017 Simon Dickson
// 
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use data::schema::{movies, tv_shows, tv_series, tv_episodes};
use chrono::prelude::*;

#[derive(Queryable, AsChangeset)]
pub struct Movie {
    pub id: i32,
    pub title: String,
    pub formatted_title: String,
    pub file_path: String,
    pub created_date: NaiveDateTime,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
}

#[derive(Insertable)]
#[table_name="movies"]
pub struct NewMovie<'a> {
    pub title: &'a str,
    pub formatted_title: &'a str,
    pub file_path: &'a str,
    pub created_date: NaiveDateTime,
}

#[derive(Queryable)]
pub struct TvShow {
    pub id: i32,
    pub title: String,
    pub created_date: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name="tv_shows"]
pub struct NewTvShow<'a> {
    pub title: &'a str,
    pub created_date: NaiveDateTime,
}

#[derive(Queryable)]
pub struct TvSeries {
    pub id: i32,
    pub series_number: i32,
    pub tv_show_id: i32,
    pub created_date: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name="tv_series"]
pub struct NewTvSeries {
    pub series_number: i32,
    pub tv_show_id: i32,
    pub created_date: NaiveDateTime,
}

#[derive(Queryable)]
pub struct TvEpisode {
    pub id: i32,
    pub tv_series_id: i32,
    pub episode_number: i32,
    pub file_path: String,
    pub created_date: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name="tv_episodes"]
pub struct NewTvEpisode<'a> {
    pub tv_series_id: i32,
    pub episode_number: i32,
    pub file_path: &'a str,
    pub created_date: NaiveDateTime,
}
