// Copyright (c) 2017 Simon Dickson
// 
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::io;
use std::path::Path;

use rocket::Route;
use rocket_contrib::Json;

use data::init::establish_connection;
use data::movies::{page_movies, get_movie};
use partial_file::{serve_partial, PartialFile};

#[derive(Serialize)]
pub struct Movie {
    pub title: String,
    pub background_image: String,
    pub card_image: String,
    pub video_url: String
}

#[derive(FromForm)]
pub struct PageRequest {
    page: Option<i64>,
    count: Option<i64>
}

#[get("/")]
pub fn all_movies_root() -> Json {
    all_movies(PageRequest{ page: None, count: None })
}

#[get("/?<page_request>")]
pub fn all_movies(page_request: PageRequest) -> Json {
    let conn = establish_connection();
    let page = match page_request.page { Some(v) => v, None => 0 };
    let count = match page_request.count { Some(v) => v, None => 10 };
    
    let movies = page_movies(&conn, page, count);

    let ip_address = env!("ROCKET_ADDRESS");
    let port = env!("ROCKET_PORT");
    
    Json(json!({
        "results": movies.into_iter().map(|m| Movie { 
            title: m.title,
            background_image: "".to_owned(), //m.background_image,
            card_image: "".to_owned(), //m.card_image,
            video_url: format!("http://{}:{}/api/movies/play/{}", ip_address, port, m.id).to_owned()
        }).collect::<Vec<_>>(),
    }))
}

#[get("/play/<movie_id>")]
pub fn play_movie(movie_id: i32) -> io::Result<PartialFile>  {
    let conn = establish_connection();
    let movie = get_movie(&conn, movie_id as i64);
    serve_partial(Path::new(&movie.file_path))
}

pub fn routes() -> Vec<Route> {
    routes![all_movies_root, all_movies, play_movie]
}
