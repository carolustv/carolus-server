// Copyright (c) 2017 Simon Dickson
// 
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[macro_use] extern crate diesel_codegen;
#[macro_use] extern crate diesel;
extern crate chrono;

pub mod init;
pub mod schema;
pub mod models;
pub mod movies;
