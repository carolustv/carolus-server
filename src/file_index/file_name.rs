// Copyright (c) 2017 Simon Dickson
// 
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::path::Path;
use std::process::Command;

use failure::Error;
use serde_json;

#[derive(Deserialize)]
pub struct Movie { 
    pub title: String,
    pub year: Option<i32>,
}

pub fn parse_movie(path: &Path) -> Result<Movie, Error> {
    let filename = path.file_stem().unwrap().to_str().unwrap();
    let output =
        Command::new("guessit")
            .arg("--json")
            .arg(filename)
            .output()
            .expect("failed to execute process");

    let m: Movie = serde_json::from_str(&String::from_utf8_lossy(&output.stdout))?;

    Ok(m)
}

#[test]
fn a_clockwork_orange(){
    match parse_movie(Path::new("A Clockwork Orange (1971).mkv")) {
        Ok(Movie { title, year: Some (year) }) => {
            assert_eq!("A Clockwork Orange", title);
            assert_eq!(1971, year);
        }
        result => assert!(false, result)
    }
}

#[test]
fn american_history_x(){
    match parse_movie(Path::new("American History X.mp4")) {
        Ok(Movie { title, year: None }) => {
            assert_eq!("American History X", title);
        }
        result => assert!(false, result)
    }
}

#[test]
fn great_escape(){
    match parse_movie(Path::new("Great Escape.m4v")) {
        Ok(Movie { title, year: None }) => {
            assert_eq!("Great Escape", title);
        }
        result => assert!(false, result)
    }
}

#[test]
fn die_hard(){
    match parse_movie(Path::new("/storage/movies/Die Hard.m4v")) {
        Ok(Movie { title, year: None }) => {
            assert_eq!("Die Hard", title);
        }
        result => assert!(false, result)
    }
}