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
#[macro_use] extern crate hyper;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate include_dir;
extern crate serde;
extern crate serde_json;
extern crate reqwest;
extern crate url;
extern crate regex;
extern crate blake2;
extern crate glob;
extern crate base64;
extern crate rocket;
extern crate chrono;
extern crate clap;
extern crate simplelog;
extern crate rusqlite;
extern crate time;

pub mod data;
pub mod partial_file;
pub mod provider;
pub mod movies;
pub mod tv;
pub mod file_index;

use clap::{Arg, App};
use log::LevelFilter;
use simplelog::TermLogger;

use file_index::index;

fn main() {
    let matches =
        App::new("carolus")
            .version("0.1.0")
            .about("Open Source Multimedia Server")
            .author("Simon Dickson")
            .arg(Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity"))
            .get_matches();

    init_logging(matches.occurrences_of("v"));

    match index::index() {
        Ok(()) => (),
        Err(err) => error!("failed to index {}", err),
    }
    
    rocket::ignite()
        .mount("/api/movies", movies::routes())
        .mount("/api/tv", tv::routes())
        .launch();
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
