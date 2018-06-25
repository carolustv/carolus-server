// Copyright (c) 2017 Simon Dickson
// 
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::io;
use std::path::Path;

use rocket::Route;
use rocket::State;
use rocket::config::Config;
use rocket_contrib::Json;

use data::init::establish_connection;
use data::tv_shows::page_tv_shows;
use data::tv_episodes::get_tv_episode;
use partial_file::{serve_partial, PartialFile};

#[derive(Serialize)]
pub struct TvSeries {
    pub series_id: i32,
    pub episodes: Vec<i32>
}

#[derive(Serialize)]
pub struct TvShows {
    pub title: String,
    pub background_image: String,
    pub card_image: String,
    pub video_url: String,
    pub series: Vec<TvSeries>,
}

#[derive(FromForm)]
pub struct PageRequest {
    page: Option<i64>,
    count: Option<i64>
}

#[get("/")]
pub fn all_tv_series_root(config: State<Config>) -> Json {
    all_tv_series(config, PageRequest{ page: None, count: None })
}

#[get("/?<page_request>")]
pub fn all_tv_series(config: State<Config>, page_request: PageRequest) -> Json {
    let conn = establish_connection();
    let page = page_request.page.unwrap_or(10);
    let count = page_request.count.unwrap_or(10);
    
    let tv_shows = page_tv_shows(&conn, page, count);
    
    Json(json!({
        "results": tv_shows.into_iter().map(|m| TvShows { 
            title: m.title,
            background_image: "".to_owned(),
            card_image: "".to_owned(),
            video_url: format!("http://{}:{}/api/tv/play/{}", config.address, config.port, m.id).to_owned(),
            series: vec![]
        }).collect::<Vec<_>>(),
    }))
}

#[get("/play/<tv_show_id>/<series_id>/<episode_id>")]
pub fn play_episode(tv_show_id: i32, series_id: i32, episode_id: i32) -> io::Result<PartialFile>  {
    let conn = establish_connection();
    let movie = get_tv_episode(&conn, episode_id as i64);
    serve_partial(Path::new(&movie.file_path))
}

pub fn routes() -> Vec<Route> {
    routes![all_tv_series_root, all_tv_series, play_episode]
}
