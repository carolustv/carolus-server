use std::env;
use std::result::Result;
use std::collections::vec_deque::VecDeque;

use chrono::Date;
use hyper::header;
use chrono::offset::Utc;
use failure::Error;
use reqwest::Client;
use url::Url;

header! { (XRateLimitLimit, "X-RateLimit-Limit") => [usize] }
header! { (XRateLimitRemaining, "X-RateLimit-Remaining") => [usize] }
header! { (XRateLimitReset, "X-RateLimit-Reset") => [u64] }

#[derive(Deserialize)]
pub struct Response<T> {
    pub results: VecDeque<T>
}

#[derive(Deserialize)]
pub struct Movie {
    pub id: i32,
    pub title: String,
    pub poster_path: String,
    pub overview: String,
    pub backdrop_path: String,
    #[serde(with = "standard_date_format")]
    pub release_date: Date<Utc>,
}

#[derive(Deserialize)]
pub struct TvShow {
    pub id: i32,
    pub name: String,
    pub poster_path: String,
    pub overview: String,
    pub backdrop_path: String,
    #[serde(with = "standard_date_format")]
    pub first_air_date: Date<Utc>,
}

pub fn find_movie(client: Client, movie_name: &str, year: i32) -> Result<Response<Movie>, Error> {
    let key = &*env::var("THE_MOVIE_DB_API_KEY")?;
    let url = Url::parse_with_params("https://api.themoviedb.org/3/search/movies",
                &[("api_key", key), ("query", movie_name), ("year", &*year.to_string())])?;
    Ok(client.get(url).send()?.json()?)
}

pub fn find_tv_show(client: Client, tv_show_name: &str, year: i32) -> Result<Response<TvShow>, Error> {
    let key = &*env::var("THE_MOVIE_DB_API_KEY")?;
    let url = Url::parse_with_params("https://api.themoviedb.org/3/search/tv",
                &[("api_key", key), ("query", tv_show_name), ("first_air_date_year", &*year.to_string())])?;
    Ok(client.get(url).send()?.json()?)
}

mod standard_date_format {
    use chrono::{Date, TimeZone};
    use chrono::offset::Utc;
    use serde::{self, Deserialize, Deserializer};

    const FORMAT: &'static str = "%Y-%m-%d";
    
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Date<Utc>, D::Error>
        where D: Deserializer<'de>
    {
        let s = String::deserialize(deserializer)?;
        let x = Utc.datetime_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?;
        Ok(x.date())
    }
}
