// Copyright (c) 2017 Simon Dickson
// 
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![feature(custom_derive, decl_macro, plugin)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate log;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate failure;
#[macro_use] extern crate lazy_static;
extern crate regex;
extern crate glob;
extern crate rocket;
extern crate clap;
extern crate simple_logger;

mod data;
mod file_index;
mod media_api;
mod partial_file;

use std::sync::Arc;
use std::time::Instant;

use clap::{Arg, App};
use failure::Error;
use log::Level;

use file_index::index;

fn main() -> Result<(), Error> {
    let matches =
        App::new("carolus")
            .version("0.1.0")
            .about("Open Source Multimedia Server")
            .author("Simon Dickson")
            .arg(Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity"))
            .arg(Arg::with_name("movie_path")
                .short("mp")
                .env("CAROLUS_MOVIES_PATH")
                .help("Sets the movie directory"))
            .arg(Arg::with_name("tv_path")
                .short("tp")
                .env("CAROLUS_TV_PATH")
                .help("Sets the tv directory"))
            .get_matches();

    init_logging(matches.occurrences_of("v"))?;

    let instant = Instant::now();

    let (movies, tv) = index::index(matches.value_of("movie_path"), matches.value_of("tv_path"))?;
    
    println!("Indexing took: {:?}", instant.elapsed());

    rocket::ignite()
        .manage(Arc::new(movies))
        .manage(Arc::new(tv))
        .mount("/api/movies", media_api::movie_routes())
        .mount("/api/tv", media_api::tv_routes())
        .launch();

    Ok(())
}

fn init_logging(level: u64) -> Result<(), Error> {
    let log_level =
        match level {
            0 => Level::Warn,
            1 => Level::Info,
            2 => Level::Debug,
            _ => Level::Trace,
        };

    simple_logger::init_with_level(log_level)?;
    Ok(())
}
