mod themoviedb;
use failure::Error;
use reqwest::Client;

pub struct MovieData {
    pub title: String,
    pub overview: String,
    pub card_image: String,
    pub background_image: String,
}

pub struct TvShowData {
    pub title: String,
    pub overview: String,
    pub card_image: String,
    pub background_image: String,
}

pub fn look_up_movie(movie_name: String, year: i32) -> Result<MovieData, Error> {
    let client = Client::new();
    let mut results = themoviedb::find_movie(client, movie_name, year)?.results;
    let movie = results.pop_front().ok_or(format_err!("exists"))?;
    Ok(MovieData{
        title: movie.title,
        overview: movie.overview,
        background_image: movie.backdrop_path,
        card_image: movie.poster_path,
    })
}


pub fn look_up_tv_show(tv_show_name: String, year: i32) -> Result<MovieData, Error> {
    let client = Client::new();
    let mut results = themoviedb::find_tv_show(client, tv_show_name, year)?.results;
    let tv_show = results.pop_front().ok_or(format_err!("exists"))?;
    Ok(MovieData{
        title: tv_show.title,
        overview: tv_show.overview,
        background_image: tv_show.backdrop_path,
        card_image: tv_show.poster_path,
    })
}
