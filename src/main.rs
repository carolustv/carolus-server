// Copyright (c) 2017 Simon Dickson
// 
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
#![feature(custom_derive)]
#![feature(plugin)]
#![feature(decl_macro)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate log;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate failure;
#[macro_use] extern crate lazy_static;
extern crate serde;
extern crate regex;
extern crate glob;
extern crate rocket;
extern crate clap;
extern crate simplelog;

pub mod data;
pub mod partial_file;
pub mod movies;
pub mod tv;
pub mod file_index;

use std::sync::Arc;
use std::time::Instant;

use clap::{Arg, App};
use failure::Error;
use log::LevelFilter;
use simplelog::TermLogger;

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

    init_logging(matches.occurrences_of("v"));

    let instant = Instant::now();

    let (movies, tv) = index::index(matches.value_of("movie_path"), matches.value_of("tv_path"))?;
    
    println!("Indexing took: {:?}", instant.elapsed());

    rocket::ignite()
        .manage(Arc::new(movies))
        .manage(Arc::new(tv))
        .mount("/api/movies", movies::routes())
        .mount("/api/tv", tv::routes())
        .launch();

    Ok(())
}

fn init_logging(level: u64) {
    let log_filter =
        match level {
            0 => LevelFilter::Warn,
            1 => LevelFilter::Info,
            2 => LevelFilter::Debug,
            _ => LevelFilter::Trace,
        };

    TermLogger::init(log_filter, Default::default()).unwrap();
}
