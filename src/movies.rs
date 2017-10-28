// Copyright (c) 2017 Simon Dickson
// 
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::path::Path;
use std::collections::HashMap;

use iron::prelude::*;
use iron::status;
use router::Router;
use url::form_urlencoded;

use data::init::establish_connection;
use data::movies::{page_movies, get_movie};
use video::play_video;

#[derive(Serialize)]
pub struct Movie {
    pub title: String,
    pub play_path: String
}

pub fn all_movies(req: &mut Request) ->  IronResult<Response> {
    let conn = establish_connection();
    let (page, count) = match req.url.query() {
        Some (query) => {
            let parse: HashMap<_,_> = form_urlencoded::parse(query.as_bytes()).into_owned().collect();
            let page = match parse.get("page") { Some (x) => x.parse::<i64>().unwrap(), None => 0 };
            let count = match parse.get("count") { Some (x) => x.parse::<i64>().unwrap(), None => 10 };
            (page, count)
        },
        None => (0, 10)
    };
    
    let movies = page_movies(&conn, page, count);
    let result = json!({
        "results": movies.into_iter().map(|m| Movie { title: m.title, play_path: url_for!(req, "play", "movie_id" => m.id.to_string()).to_string()  }).collect::<Vec<_>>(),
    });
    Ok(Response::with((status::Ok, result.to_string())))
}

pub fn play_movie(req: &mut Request) ->  IronResult<Response> {
    let conn = establish_connection();
    let movie_id = req.extensions.get::<Router>().unwrap().find("movie_id").unwrap_or("/").parse::<i64>().unwrap();
    let movie = get_movie(&conn, movie_id);
    play_video(req, Path::new(&movie.file_path))
}

pub fn register() -> Router {
    router!{
        all_movies: get "/" => all_movies,
        play: get "/play/:movie_id" => play_movie,
    }
}
