// Copyright (c) 2017 Simon Dickson
// 
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::process::Command;

use failure::Error;
use serde_json;

#[derive(Deserialize)]
pub struct Movie { 
    pub title: String,
    pub year: Option<i32>,
}

pub fn parse(filepath: &str) -> Result<Movie, Error> {
    let output =
        Command::new("guessit")
            .args(&["-t", "movie"])
            .arg("--json")
            .arg(filepath)
            .output()
            .expect("failed to execute process");

    Ok(serde_json::from_str(&String::from_utf8_lossy(&output.stdout))?)
}

#[test]
fn a_clockwork_orange(){
    match parse("A Clockwork Orange (1971).mkv") {
        Ok(Movie { title, year: Some (year) }) => {
            assert_eq!("A Clockwork Orange", title);
            assert_eq!(1971, year);
        }
        result => assert!(false, result)
    }
}

#[test]
fn american_history_x(){
    match parse("American History X.mp4") {
        Ok(Movie { title, year: None }) => {
            assert_eq!("American History X", title);
        }
        result => assert!(false, result)
    }
}

#[test]
fn great_escape(){
    match parse("Great Escape.m4v") {
        Ok(Movie { title, year: None }) => {
            assert_eq!("Great Escape", title);
        }
        result => assert!(false, result)
    }
}

#[test]
fn die_hard(){
    match parse("/storage/movies/Die Hard.m4v") {
        Ok(Movie { title, year: None }) => {
            assert_eq!("Die Hard", title);
        }
        result => assert!(false, result)
    }
}