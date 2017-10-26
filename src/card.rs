// Copyright (c) 2017 Simon Dickson
// 
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::error::Error;
use std::fmt;

use iron::prelude::*;
use iron::headers::{AcceptRanges, RangeUnit, Range};
use iron::modifiers::Header;
use iron::status;
use router::Router;

use data::init::establish_connection;
use data::movies::page_movies;

pub fn all_cards(req: &mut Request) ->  IronResult<Response> {
    let cheese = req.params.query;
    let conn = establish_connection();
    Ok(Response::with(status::RangeNotSatisfiable))
}

pub fn register() -> Router {
    router!{
        all_cards: get "/" => all_cards,
    }
}
