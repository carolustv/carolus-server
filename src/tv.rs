// Copyright (c) 2017 Simon Dickson
// 
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::io;
use std::path::Path;

use failure::Error;
use rocket::Route;
use rocket_contrib::Json;

use data::init::establish_connection;
use data::tv_shows::page_tv_shows;
use data::tv_episodes::get_tv_episode;
use partial_file::{serve_partial, PartialFile};

#[derive(Serialize)]
pub struct TvSeries {
    pub series: i32,
    pub episodes: Vec<i32>
}

#[derive(Serialize)]
pub struct TvShows {
    pub title: String,
    pub background_image: String,
    pub card_image: String,
    pub series: Vec<TvSeries>,
}

#[derive(FromForm)]
pub struct PageRequest {
    page: Option<i64>,
    count: Option<i64>
}

#[get("/")]
pub fn all_tv_shows_root() -> Result<Json, Error> {
    all_tv_shows(PageRequest{ page: None, count: None })
}

#[get("/?<page_request>")]
pub fn all_tv_shows(page_request: PageRequest) -> Result<Json, Error> {
    let conn = establish_connection()?;
    let page = page_request.page.unwrap_or(0);
    let count = page_request.count.unwrap_or(1);
    
    let tv_shows = page_tv_shows(&conn, page, count);
    
    Ok(Json(json!({
        "results": tv_shows.into_iter().map(|(tv_show, tv_series)| TvShows { 
            title: tv_show.title,
            background_image: "".to_owned(),
            card_image: "".to_owned(),
            series: tv_series.iter().map(|(tv_series, episodes)| TvSeries { series: tv_series.series_number, episodes: episodes.iter().map(|e|e.episode_number).collect() }).collect()
        }).collect::<Vec<_>>(),
    })))
}

#[get("/play/<episode_id>")]
pub fn play_episode(episode_id: i32) -> Result<io::Result<PartialFile>, Error>  {
    let conn = establish_connection()?;
    let movie = get_tv_episode(&conn, episode_id as i64);
    Ok(serve_partial(Path::new(&movie.file_path)))
}

pub fn routes() -> Vec<Route> {
    routes![all_tv_shows_root, all_tv_shows, play_episode]
}
