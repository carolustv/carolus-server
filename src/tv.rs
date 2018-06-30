// Copyright (c) 2017 Simon Dickson
// 
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::io;
use std::path::Path;
use std::sync::Arc;

use failure::Error;
use rocket::{Route, State};
use rocket_contrib::Json;

use data::tv::{get_episode, page_tv_shows, TvShow};
use partial_file::{serve_partial, PartialFile};

#[derive(Serialize)]
pub struct TvSeriesJson {
    pub series: u16,
    pub episodes: Vec<u16>
}

#[derive(Serialize)]
pub struct TvShowJson {
    pub title: String,
    pub background_image: String,
    pub card_image: String,
    pub series: Vec<TvSeriesJson>,
}

#[derive(FromForm)]
pub struct PageRequest {
    page: Option<i64>,
    count: Option<i64>
}

#[get("/")]
pub fn all_tv_shows_root(state: State<Arc<Vec<TvShow>>>) -> Result<Json, Error> {
    all_tv_shows(state, PageRequest{ page: None, count: None })
}

#[get("/?<page_request>")]
pub fn all_tv_shows(state: State<Arc<Vec<TvShow>>>, page_request: PageRequest) -> Result<Json, Error> {
    let page = page_request.page.unwrap_or(0);
    let count = page_request.count.unwrap_or(10);
    
    let tv_shows = page_tv_shows(state.inner(), page, count).ok_or(format_err!("failed to get tv shows"))?;
    
    Ok(Json(json!({
        "results": tv_shows.into_iter().map(|tv_show| TvShowJson { 
            title: tv_show.title.to_owned(),
            background_image: "".to_owned(),
            card_image: "".to_owned(),
            series: tv_show.series.iter().map(|series| TvSeriesJson { series: series.series_number, episodes: series.episodes.iter().map(|e|e.episode_number).collect() }).collect()
        }).collect::<Vec<_>>(),
    })))
}

#[derive(FromForm)]
pub struct PlayRequest {
    year: Option<u16>,
}

#[get("/play/<show_title>/<series_number>/<episode_number>")]
pub fn play_episode_without_year(state: State<Arc<Vec<TvShow>>>, show_title: String, series_number: u16, episode_number: u16) -> Result<io::Result<PartialFile>, Error> {
    play_episode(state, show_title, series_number, episode_number, PlayRequest{ year: None })
}

#[get("/play/<show_title>/<series_number>/<episode_number>?<play_request>")]
pub fn play_episode(state: State<Arc<Vec<TvShow>>>, show_title: String, series_number: u16, episode_number: u16, play_request: PlayRequest) -> Result<io::Result<PartialFile>, Error>  {
    let (_, _, episode) = get_episode(state.inner(), &show_title, play_request.year, series_number, episode_number).ok_or(format_err!("failed to get tv shows"))?;
    Ok(serve_partial(Path::new(&episode.file_path)))
}

pub fn routes() -> Vec<Route> {
    routes![all_tv_shows_root, all_tv_shows, play_episode_without_year, play_episode]
}
