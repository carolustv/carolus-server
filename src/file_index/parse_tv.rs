// Copyright (c) 2017 Simon Dickson
// 
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::path::Path;

use failure::Error;
use regex::Regex;

#[derive(Debug)]
pub struct Tv<'a> {
    pub title: &'a str,
    pub season: i32,
    pub episode: i32,
    pub year: Option<i32>,
}

pub fn parse<'a>(search_path: &Path, path: &'a Path) -> Result<Tv<'a>, Error> {
    let (title, year) = parse_title(search_path, path)?;
    let (season, episode) = parse_season_and_episode(path)?;

    Ok(Tv {
        title: title,
        season: season,
        episode: episode,
        year: year
    })
}

fn parse_season_and_episode(path: &Path) -> Result<(i32, i32), Error> {
    lazy_static! {
        static ref SEASON_EPISODE_FORMAT_1: Regex = Regex::new(r"S(\d\d?)E(\d\d?)").unwrap();
    }

    let file_name =
        path.file_name().ok_or(format_err!("failed to parse filename"))?
            .to_str().ok_or(format_err!("failed to parse filename"))?;
            
    let cap = SEASON_EPISODE_FORMAT_1.captures_iter(file_name).nth(0).ok_or(format_err!("could not parse season or episode number"))?;

    let season = cap.get(1).map(|m| m.as_str()).ok_or(format_err!("could not parse season or episode number"))?.parse::<i32>()?;
    let episode = cap.get(2).map(|m| m.as_str()).ok_or(format_err!("could not parse season or episode number"))?.parse::<i32>()?;

    Ok((season, episode))
}

fn parse_title<'a>(base_path: &Path, path: &'a Path) -> Result<(&'a str, Option<i32>), Error> {
    lazy_static! {
        static ref TITLE_FORMAT_1: Regex = Regex::new(r"([^']+)\s+\((\d{4})\)").unwrap();
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
            Ok((folder_name, None))
        },
    }
}

#[test]
fn hello_world(){
    match parse(Path::new("/storage/tv/"), Path::new("/storage/tv/Hello World/Season 1/S02E01.m4v")) {
        Ok(Tv { title: "Hello World", season: 2, episode: 1, year: None }) => (),
        result => assert!(false, "{:?}", result)
    }
}


#[test]
fn south_park(){
    match parse(Path::new("/storage/tv/"), Path::new("/storage/tv/South Park/Season 1/S01E02.m4v")) {
        Ok(Tv { title: "South Park", season: 1, episode: 2, year: None }) => (),
        result => assert!(false, "{:?}", result)
    }
}

#[test]
fn house_of_cards(){
    match parse(Path::new("/storage/tv/"), Path::new("/storage/tv/House of Cards (1990)/Season 2/S02E04.m4v")) {
        Ok(Tv { title: "House of Cards", season: 2, episode: 4, year: Some(1990) }) => (),
        result => assert!(false, "{:?}", result)
    }
}

#[test]
fn futurama(){
    match parse(Path::new("/storage/tv/"), Path::new("/storage/tv/Futurama/S04E08.m4v")) {
        Ok(Tv { title: "Futurama", season: 4, episode: 8, year: None }) => (),
        result => assert!(false, "{:?}", result)
    }
}
