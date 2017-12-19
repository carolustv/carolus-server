use std::env;
use std::result::Result;

use reqwest::Client;
use url::Url;
use chrono::Date;
use chrono::offset::Utc;
use failure::Error;

#[derive(Deserialize)]
pub struct Response<T> {
    pub results: Vec<T>
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

pub fn find_movie(client: Client, movie_name: String, year: i32) -> Result<Response<Movie>, Error> {
    let key = env::var("THE_MOVIE_DB_API_KEY")?;
    let url = Url::parse_with_params("https://api.themoviedb.org/3/search/movies",
                &[("api_key", key), ("query", movie_name), ("year", year.to_string())])?;
    Ok(client.get(url).send()?.json()?)
}

pub fn find_tv_show(client: Client, tv_show_name: String, year: i32) -> Result<Response<Movie>, Error> {
    let key = env::var("THE_MOVIE_DB_API_KEY")?;
    let url = Url::parse_with_params("https://api.themoviedb.org/3/search/tv",
                &[("api_key", key), ("query", tv_show_name), ("first_air_date_year", year.to_string())])?;
    Ok(client.get(url).send()?.json()?)
}

pub fn find_tv_series(client: Client, tv_show_id: i32, series_id: i32) -> Result<Response<Movie>, Error> {
    let key = env::var("THE_MOVIE_DB_API_KEY")?;
    let url = Url::parse_with_params(&format!("https://api.themoviedb.org/3/tv/{}/season/{}", tv_show_id, series_id),
                &[("api_key", key)])?;
    Ok(client.get(url).send()?.json()?)
}

pub fn find_tv_episode(client: Client, tv_show_id: i32, series_id: i32, episode_id: i32) -> Result<Response<Movie>, Error> {
    let key = env::var("THE_MOVIE_DB_API_KEY")?;
    let url = Url::parse_with_params(&format!("https://api.themoviedb.org/3/tv/{}/season/{}/episode/{}", tv_show_id, series_id, episode_id),
                &[("api_key", key)])?;
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
