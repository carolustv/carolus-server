// Copyright (c) 2017 Simon Dickson
// 
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::path::Path;
use std::error::Error;
use std::fmt;

use iron::prelude::*;
use iron::headers::{AcceptRanges, RangeUnit, Range};
use iron::modifiers::Header;
use iron::status;
use router::Router;

use data::init::establish_connection;
use data::movies::get_movie;
use partial_file::PartialFile;

pub fn video(req: &mut Request) ->  IronResult<Response> {
    let conn = establish_connection();
    let video_id = req.extensions.get::<Router>().unwrap().find("video_id").unwrap_or("/").parse::<i64>().unwrap();
    let movie = get_movie(&conn, video_id);
    let path = Path::new(&movie.file_path);
    
    match path.exists() {
        false => Err(IronError::new(NoFile, status::NotFound)),
        true => {
            let accept_range_header = Header(AcceptRanges(vec![RangeUnit::Bytes]));
            let range_req_header = req.headers.get::<Range>().map(|h|{h.clone()});
            match range_req_header {
                None => {
                    Ok(Response::with((status::Ok, path, accept_range_header)))
                },
                Some(range) => {
                    match range {
                        Range::Bytes(vec_range) => {
                            let partial_file = PartialFile::from_path(&path, vec_range);
                            Ok(Response::with((status::Ok, partial_file, accept_range_header)))
                        },
                        _ => Ok(Response::with(status::RangeNotSatisfiable))
                    }
                }
            }
        },
    }
}

pub fn register() -> Router {
    router!{
        video: get ":video_id" => video
    }
}

#[derive(Debug)]
pub struct NoFile;

impl Error for NoFile {
    fn description(&self) -> &str { "File not found" }
}

impl fmt::Display for NoFile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.description())
    }
}