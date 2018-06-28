// Copyright (c) 2017 Simon Dickson
// 
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::path::Path;

use failure::Error;
use regex::Regex;

#[derive(Debug)]
pub struct Movie<'a> { 
    pub title: &'a str,
    pub year: Option<i32>,
}

pub fn parse<'a>(search_path: &Path, path: &'a Path) -> Result<Movie<'a>, Error> {
    let (title, year) = parse_title(search_path, path)?;

    Ok(Movie {
        title: title,
        year: year
    })
}

fn parse_title<'a>(base_path: &Path, path: &'a Path) -> Result<(&'a str, Option<i32>), Error> {
    lazy_static! {
        static ref TITLE_FORMAT_1: Regex = Regex::new(r"([^']+)\s+\((\d{4})\)").unwrap();
    }
    lazy_static! {
        static ref TITLE_FORMAT_2: Regex = Regex::new(r"([^']+)\.").unwrap();
    }

    let folder_name =
        path.strip_prefix(base_path)?.components().next().ok_or(format_err!("failed to parse folder"))?
            .as_os_str().to_str().ok_or(format_err!("failed to parse folder"))?;
            
    match TITLE_FORMAT_1.captures_iter(folder_name).nth(0) {
        Some (cap) => {
            let title = cap.get(1).map(|m| m.as_str()).ok_or(format_err!("failed to parse title"))?;
            let year = cap.get(2).map(|m| m.as_str()).ok_or(format_err!("failed to parse year"))?.parse::<i32>()?;
            Ok((title, Some(year)))
        },
        None => {
            match TITLE_FORMAT_2.captures_iter(folder_name).nth(0) {
                Some (cap) => {
                    let title = cap.get(1).map(|m| m.as_str()).ok_or(format_err!("failed to parse title"))?;
                    Ok((title, None))
                },
                None => {
                    Ok((folder_name, None))
                },
            }
        },
    }
}

#[test]
fn a_clockwork_orange(){
    match parse(Path::new("/storage/movies/"), Path::new("/storage/movies/A Clockwork Orange (1971).mkv")) {
        Ok(Movie { title: "A Clockwork Orange", year: Some (1971) }) => (),
        result => assert!(false, "{:?}", result)
    }
}

#[test]
fn american_history_x(){
    match parse(Path::new("/storage/movies/"), Path::new("/storage/movies/American History X.mp4")) {
        Ok(Movie { title: "American History X", year: None }) => (),
        result => assert!(false, "{:?}", result)
    }
}

#[test]
fn great_escape(){
    match parse(Path::new("/storage/movies/"), Path::new("/storage/movies/Great Escape.m4v")) {
        Ok(Movie { title: "Great Escape", year: None }) => (),
        result => assert!(false, "{:?}", result)
    }
}

#[test]
fn die_hard(){
    match parse(Path::new("/storage/movies/"), Path::new("/storage/movies/Die Hard.m4v")) {
        Ok(Movie { title: "Die Hard", year: None }) => (),
        result => assert!(false, "{:?}", result)
    }
}