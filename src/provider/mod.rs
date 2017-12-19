mod themoviedb;
use failure::Error;
use reqwest::Client;

pub struct MovieData {
    pub title: String,
    pub overview: String,
    pub card_image: String,
    pub background_image: String,
}

//pub fn look_up_movie(movie_name: String, year: i32) -> Result<MovieData, Error> {
//    let client = Client::new();
//    let theMovieDbData = themoviedb::find_movie(client, movie_name, year)?.results;
//    Ok(MovieData{
//        title: theMovieDbData.title,
//        overview: theMovieDbData.overview,
//        background_image: theMovieDbData.backdrop_path,
//        card_image: theMovieDbData.poster_path,
//    })
//}
