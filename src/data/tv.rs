// Copyright (c) 2017 Simon Dickson
// 
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[derive(PartialEq, Debug)]
pub struct TvShow {
    pub title: String,
    pub year: Option<u16>,
    pub series: Vec<TvSeries>
}

#[derive(PartialEq, Debug)]
pub struct TvSeries {
    pub series_number: u16,
    pub episodes: Vec<TvEpisode>
}

#[derive(PartialEq, Debug)]
pub struct TvEpisode {
    pub episode_number: u16,
    pub file_path: String,
}

pub fn page_tv_shows<'a>(tv_shows: &'a Vec<TvShow>, page: i64, count: i64) -> Option<&'a [TvShow]> {
    tv_shows.chunks(count as usize).skip(page as usize).next()
}

pub fn get_episode<'a> (tv_shows: &'a Vec<TvShow>, title: &str, year: Option<u16>, series: u16, episode: u16) -> Option<(&'a TvShow, &'a TvSeries, &'a TvEpisode)> {
    let tv_show = tv_shows.iter().find(|s|s.title == title && s.year == year)?;
    let series = tv_show.series.iter().find(|s|s.series_number == series)?;
    let episode = series.episodes.iter().find(|s|s.episode_number == episode)?;
    Some((tv_show, series, episode))
}
