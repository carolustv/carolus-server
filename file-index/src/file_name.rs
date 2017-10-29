// Copyright (c) 2017 Simon Dickson
// 
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::io;
use std::path::Path;

use regex::Regex;

pub enum ParseResult {
    Movie { title: String, year: Option<i32> }
}

pub fn parse(path: &Path) -> io::Result<ParseResult> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"([^']+)\s+\((\d{4})\)").unwrap();
    }
    let filename = path.file_stem().unwrap().to_str().unwrap();
    match RE.captures(filename) {
        Some(caps) => {
            let title = caps.get(1).unwrap().as_str();
            let year = caps.get(2).map(|x|x.as_str().parse::<i32>().ok()).unwrap_or_default();
            Ok(ParseResult::Movie { title: title.to_owned(), year: year })
        }
        _ => Ok(ParseResult::Movie { title: filename.to_owned(), year: None }),
    }
}

#[test]
fn a_clockwork_orange(){
    match parse(Path::new("A Clockwork Orange (1971).mkv")) {
        Ok(ParseResult::Movie { title, year: Some (year) }) => {
            assert_eq!("A Clockwork Orange", title);
            assert_eq!(1971, year);
        }
        result => assert!(false, result)
    }
}

#[test]
fn american_history_x(){
    match parse(Path::new("American History X.mp4")) {
        Ok(ParseResult::Movie { title, year: None }) => {
            assert_eq!("American History X", title);
        }
        result => assert!(false, result)
    }
}

#[test]
fn great_escape(){
    match parse(Path::new("Great Escape.m4v")) {
        Ok(ParseResult::Movie { title, year: None }) => {
            assert_eq!("Great Escape", title);
        }
        result => assert!(false, result)
    }
}

#[test]
fn die_hard(){
    match parse(Path::new("/storage/movies/Die Hard.m4v")) {
        Ok(ParseResult::Movie { title, year: None }) => {
            assert_eq!("Die Hard", title);
        }
        result => assert!(false, result)
    }
}
