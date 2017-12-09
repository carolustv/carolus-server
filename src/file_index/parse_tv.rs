// Copyright (c) 2017 Simon Dickson
// 
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::process::Command;

use failure::Error;
use serde_json;

#[derive(Deserialize)]
pub struct Tv { 
    pub title: String,
    pub season: i32,
    pub episode: i32,
    pub year: Option<i32>,
}

pub fn parse(filepath: &str) -> Result<Tv, Error> {
    let output =
        Command::new("guessit")
            .args(&["-t", "episode"])
            .arg("--json")
            .arg(format!("'{}'", filepath))
            .output()
            .expect("failed to execute process");
            
    Ok(serde_json::from_str(&String::from_utf8_lossy(&output.stdout))?)
}

#[test]
fn south_park(){
    match parse("/storage/tv/South Park/Season 1/S01E01.m4v") {
        Ok(Tv { title, season: 1, episode: 1, year: None }) => {
            assert_eq!("South Park", title);
        }
        result => assert!(false, result)
    }
}