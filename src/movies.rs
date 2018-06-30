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

use data::movies::{page_movies, get_movie, Movie};
use partial_file::{serve_partial, PartialFile};

#[derive(Serialize)]
pub struct MovieJson {
    pub title: String,
    pub year: Option<u16>,
    pub background_image: String,
    pub card_image: String
}

#[derive(FromForm)]
pub struct PageRequest {
    page: Option<i64>,
    count: Option<i64>
}

#[get("/")]
pub fn all_movies_root(state: State<Arc<Vec<Movie>>>) -> Result<Json, Error> {
    all_movies(state, PageRequest{ page: None, count: None })
}

#[get("/?<page_request>")]
pub fn all_movies(state: State<Arc<Vec<Movie>>>, page_request: PageRequest) -> Result<Json, Error> {
    let page = page_request.page.unwrap_or(0);
    let count = page_request.count.unwrap_or(10);
    
    let movies = page_movies(state.inner(), page, count).ok_or(format_err!("expected shows"))?;
    
    Ok(Json(json!({
        "results": movies.into_iter().map(|m| MovieJson {
            title: m.title.to_owned(),
            year: m.year,
            background_image: "".to_owned(),
            card_image: "".to_owned()
        }).collect::<Vec<_>>(),
    })))
}

#[derive(FromForm)]
pub struct PlayRequest {
    year: Option<u16>,
}

#[get("/play/<title>")]
pub fn play_movie_without_year(state: State<Arc<Vec<Movie>>>, title: String) -> Result<io::Result<PartialFile>, Error> {
    play_movie(state, title, PlayRequest{ year: None })
}

#[get("/play/<title>?<play_request>")]
pub fn play_movie(state: State<Arc<Vec<Movie>>>, title: String, play_request: PlayRequest) -> Result<io::Result<PartialFile>, Error>  {
    let movie = get_movie(state.inner(), &title, play_request.year).ok_or(format_err!("movie not found"))?;
    Ok(serve_partial(Path::new(&movie.file_path)))
}

pub fn routes() -> Vec<Route> {
    routes![all_movies_root, all_movies, play_movie_without_year, play_movie]
}
