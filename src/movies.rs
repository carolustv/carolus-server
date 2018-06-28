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
use data::movies::{page_movies, get_movie};
use partial_file::{serve_partial, PartialFile};

#[derive(Serialize)]
pub struct Movie {
    pub id: i32,
    pub title: String,
    pub background_image: String,
    pub card_image: String
}

#[derive(FromForm)]
pub struct PageRequest {
    page: Option<i64>,
    count: Option<i64>
}

#[get("/")]
pub fn all_movies_root() -> Result<Json, Error> {
    all_movies(PageRequest{ page: None, count: None })
}

#[get("/?<page_request>")]
pub fn all_movies(page_request: PageRequest) -> Result<Json, Error> {
    let conn = establish_connection()?;
    let page = page_request.page.unwrap_or(0);
    let count = page_request.count.unwrap_or(10);
    
    let movies = page_movies(&conn, page, count);
    
    Ok(Json(json!({
        "results": movies.into_iter().map(|m| Movie {
            id: m.id,
            title: m.title,
            background_image: "".to_owned(),
            card_image: "".to_owned()
        }).collect::<Vec<_>>(),
    })))
}

#[get("/play/<movie_id>")]
pub fn play_movie(movie_id: i32) -> Result<io::Result<PartialFile>, Error>  {
    let conn = establish_connection()?;
    let movie = get_movie(&conn, movie_id as i64);
    Ok(serve_partial(Path::new(&movie.file_path)))
}

pub fn routes() -> Vec<Route> {
    routes![all_movies_root, all_movies, play_movie]
}
