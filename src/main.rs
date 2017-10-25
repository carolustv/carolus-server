// Copyright (c) 2017 Simon Dickson
// 
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[macro_use] extern crate router;
extern crate dotenv;
extern crate iron;
extern crate mount;
extern crate data;
extern crate file_index;

mod video;
mod partial_file;

use dotenv::dotenv;
use iron::prelude::*;
use mount::Mount;

fn main() {
    dotenv().ok();
    file_index::index();
    let mut mount = Mount::new();
    mount.mount("/api/video", video::register());
    Iron::new(mount).http("localhost:3000").unwrap();
}
