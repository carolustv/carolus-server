// Copyright (c) 2017 Simon Dickson
// 
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::collections::{BTreeMap, HashMap, HashSet};
use std::ffi::OsStr;
use std::fs::read_dir;
use std::iter::FromIterator;
use std::path::Path;

use failure::Error;
use glob::glob;

use data::{Movie, TvShow, TvSeries, TvEpisode};
use file_index::parse_movie;
use file_index::parse_tv;

lazy_static! {
    static ref FILE_TYPES: HashSet<&'static str> = HashSet::from_iter(vec!["mp4", "mkv", "flv", "m4v", "avi"]);
}

fn index_movie_directory(directory: Option<&str>) -> Result<Vec<Movie>, Error> {
    match directory {
        Some (directory) => {
            let root_dir = Path::new(directory);
            let mut result = BTreeMap::new();
            for entry in read_dir(root_dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_file() && FILE_TYPES.contains(path.extension().and_then(OsStr::to_str).ok_or(format_err!("expected file"))?) {
                    match parse_movie::parse(&root_dir, &path) {
                        Ok(movie) => {
                            trace!("Found movie: {}, year: {:?}, file: {:?}", movie.title, movie.year, movie.file_path);
                            result.insert((movie.title.clone(), movie.year), movie);
                        },
                        Err(err) => warn!("Could not parse movie file: {:?}, err: {}", path, err)
                    }
                }
            }
            Ok(result.into_iter().map(|(_, v)|v).collect())
        },
        None => Ok(vec![]),
    }
}

fn index_tv_directory(directory: Option<&str>) -> Result<Vec<TvShow>, Error> {
    match directory {
        Some (directory) => {
            let root_dir = Path::new(directory);
            let mut result = BTreeMap::new();
            for entry in read_dir(root_dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    match parse_tv::parse_title(&root_dir, &path) {
                        Ok((title, year)) => {
                            match index_tv_show(title, &path) {
                                Ok(series) => {
                                    result.insert((title.to_owned(), year), TvShow { title: title.to_owned(), year, series: series });
                                },
                                Err(err) => warn!("Could not parse tv series: {:?}, err: {}", path, err),
                            }
                        },
                        Err (err) => warn!("Could not parse tv show: {:?}, err: {}", path, err),
                    }
                }
            }
            Ok(result.into_iter().map(|(_, v)|v).collect())
        },
        None => Ok(vec![]),
    }
}

fn index_tv_show(title: &str, path: &Path) -> Result<Vec<TvSeries>, Error> {
    let mut series = HashMap::new();
    for path in glob(&format!("{}/**/*.*", path.to_str().ok_or(format_err!("expected file"))?))?.filter_map(Result::ok) {
        if path.is_file() && !FILE_TYPES.contains(path.extension().and_then(OsStr::to_str).ok_or(format_err!("expected file 2"))?) {
            continue
        }
        match parse_tv::parse_season_and_episode(&path) {
            Ok((season, episode)) => {
                trace!("Found tv episode: {}, S{:02}E{:02}, file: {:?}", title, season, episode, path);
                let episode =
                    TvEpisode {
                        episode_number: episode,
                        file_path: path.to_str().ok_or(format_err!("should be a path"))?.to_owned()
                    };
                if !series.contains_key(&season) {
                    series.insert(season, vec![episode]);
                } else {
                    if let Some(value) = series.get_mut(&season) {
                        (*value).push(episode);
                    }
                }
            },
            Err(err) => warn!("Parse failed for {:?}, err: {}", path, err),
        }
    }
    Ok(series.into_iter().map(|(k, v)| TvSeries { series_number: k, episodes: v }).collect())
}

pub fn index(movie_directory: Option<&str>, tv_directory: Option<&str>) -> Result<(Vec<Movie>, Vec<TvShow>), Error> {
    let movies = index_movie_directory(movie_directory)?;
    let tv = index_tv_directory(tv_directory)?;
    Ok((movies, tv))
}
