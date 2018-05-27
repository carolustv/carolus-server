// Copyright (c) 2017 Simon Dickson
// 
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
#![feature(custom_derive)]
#![feature(plugin)]
#![feature(decl_macro)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate diesel;
#[macro_use] extern crate log;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate failure;
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

pub mod data;
pub mod partial_file;
pub mod provider;
pub mod movies;
pub mod tv;
pub mod file_index;

use file_index::index;

fn main() {
    index::index();
    rocket::ignite().mount("/api/movies", movies::routes()).launch();
}
