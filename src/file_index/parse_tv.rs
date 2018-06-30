// Copyright (c) 2017 Simon Dickson
// 
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::path::Path;

use failure::Error;
use regex::Regex;

pub fn parse_season_and_episode(path: &Path) -> Result<(u16, u16), Error> {
    lazy_static! {
        static ref SEASON_EPISODE_FORMAT_1: Regex = Regex::new(r"[Ss](\d*?)[Ee](\d*)").unwrap();
    }

    let file_name =
        path.file_name().ok_or(format_err!("failed to parse filename"))?
            .to_str().ok_or(format_err!("failed to parse filename"))?;
            
    let cap = SEASON_EPISODE_FORMAT_1.captures_iter(file_name).nth(0).ok_or(format_err!("could not parse season or episode number"))?;

    let season = cap.get(1).map(|m| m.as_str()).ok_or(format_err!("could not parse season or episode number"))?.parse::<u16>()?;
    let episode = cap.get(2).map(|m| m.as_str()).ok_or(format_err!("could not parse season or episode number"))?.parse::<u16>()?;

    Ok((season, episode))
}

pub fn parse_title<'a>(base_path: &Path, path: &'a Path) -> Result<(&'a str, Option<u16>), Error> {
    lazy_static! {
        static ref TITLE_FORMAT_1: Regex = Regex::new(r"([^']+)\s+\((\d{4})\)").unwrap();
    }

    let folder_name =
        path.strip_prefix(base_path)?.components().next().ok_or(format_err!("failed to parse folder"))?
            .as_os_str().to_str().ok_or(format_err!("failed to parse folder"))?;
            
    match TITLE_FORMAT_1.captures_iter(folder_name).nth(0) {
        Some (cap) => {
            let title = cap.get(1).map(|m| m.as_str()).ok_or(format_err!("failed to parse title"))?;
            let year = cap.get(2).map(|m| m.as_str()).ok_or(format_err!("failed to parse year"))?.parse::<u16>()?;
            Ok((title, Some(year)))
        },
        None => {
            Ok((folder_name, None))
        },
    }
}
