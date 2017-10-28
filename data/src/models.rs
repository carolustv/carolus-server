// Copyright (c) 2017 Simon Dickson
// 
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use schema::movies;
use chrono::prelude::*;

#[derive(Queryable)]
pub struct Movie {
    pub id: i32,
    pub title: String,
    pub file_path: String,
    pub created_date: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name="movies"]
pub struct NewMovie<'a> {
    pub title: &'a str,
    pub file_path: &'a str,
    pub created_date: NaiveDateTime,
}