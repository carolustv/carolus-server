// Copyright (c) 2017 Simon Dickson
// 
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[derive(PartialEq, Debug)]
pub struct Movie {
    pub title: String,
    pub year: Option<u16>,
    pub file_path: String
}

pub fn page_movies<'a>(movies: &'a Vec<Movie>, page: i64, count: i64) -> Option<&'a [Movie]> {
    movies.chunks(count as usize).skip(page as usize).next()
}

pub fn get_movie<'a>(movies: &'a Vec<Movie>, title: &str, year: Option<u16>) -> Option<&'a Movie> {
    movies.iter().find(|m|m.title == title && m.year == year)
}
