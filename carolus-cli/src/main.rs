// Copyright (c) 2017 Simon Dickson
// 
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
#![feature(custom_derive)]
#![feature(plugin)]
#![feature(decl_macro)]
#![plugin(rocket_codegen)]


#[macro_use] extern crate serde_derive;
#[macro_use] extern crate rocket_contrib;
extern crate rocket;
extern crate serde;
extern crate dotenv;
extern crate data;
extern crate file_index;

mod partial_file;
mod movies;

use dotenv::dotenv;

fn main() {
    dotenv().ok();
    file_index::index();
    rocket::ignite().mount("/api/movies", movies::routes()).launch();
}
