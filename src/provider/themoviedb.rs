use std::env;
use std::result::Result;
use std::error::Error;

use reqwest::Client;
use url::Url;
use chrono::Date;
use chrono::offset::Utc;

#[derive(Deserialize)]
pub struct Response<T> {
    pub results: Vec<T>
}

#[derive(Deserialize)]
pub struct Movie {
    pub title: String,
    pub poster_path: String,
    pub overview: String,
    pub backdrop_path: String,
    #[serde(with = "my_date_format")]
    pub release_date: Date<Utc>,
}

pub fn find_movie(client: Client, movie_name: String, year: i32) -> Result<Response<Movie>, Box<Error>> {
    let key = env::var("THE_MOVIE_DB_API_KEY")?;
    let url = Url::parse_with_params("https://api.themoviedb.org/3/search/movies",
                &[("api_key", key), ("query", movie_name), ("year", year.to_string())])?;
    Ok(client.get(url).send()?.json()?)
}

mod my_date_format {
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
