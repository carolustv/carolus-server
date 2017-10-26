// Copyright (c) 2017 Simon Dickson
// 
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate blake2;
extern crate data;
extern crate glob;
extern crate base64;


use std::{fs,env};
use std::path::PathBuf;

use base64::encode;
use blake2::{Blake2b, Digest};
use glob::glob;

use data::init::establish_connection;
use data::movies::create_movie;

fn index_movie_directory(add_movie: &Fn(&PathBuf)) {
    match env::var("CAROLUS_MOVIES_PATH") {
        Ok (directories) => {
            for directory in directories.split(",") {
                for file in glob(&format!("{}/*.mp4", &directory)).unwrap().filter_map(Result::ok) {
                    add_movie(&file);
                }
            }
        },
        Err(_) => (),
    }
}

fn hash_file(path: &PathBuf) -> String {
    let mut file = fs::File::open(&path).unwrap();
    encode(Blake2b::digest_reader(&mut file).unwrap().as_ref())
}

pub fn index() {
    let conn = establish_connection();
    index_movie_directory(&|movie|{
        let movie_name = movie.file_name().unwrap().to_str().unwrap();
        let file_path = movie.to_str().unwrap();
        let file_hash = hash_file(movie);
        create_movie(&conn, &movie_name, &file_path, &file_hash);
    });
}
