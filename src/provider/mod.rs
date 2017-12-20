mod themoviedb;
use reqwest::Client;
use diesel::sqlite::SqliteConnection;
use data::movies::{all_movies, update_movie};

pub struct MovieData {
    pub title: String,
    pub overview: String,
    pub card_image: String,
    pub background_image: String,
}

pub fn refresh_metdata(conn: &SqliteConnection) {
    let mut movies = all_movies(conn);

    for movie in movies.iter_mut() {
        let metadata = look_up_movie(&movie.title, None);
        movie.poster_path = Some(metadata.card_image);
        movie.backdrop_path = Some(metadata.background_image);
        update_movie(conn, movie);
    }
}

fn look_up_movie(movie_name: &str, year: Option<i32>) -> MovieData {
    let client = Client::new();
    let results = themoviedb::find_movie(client, movie_name.to_owned(), year).unwrap().results;
    let theMovieDbData = results.first().unwrap();
    MovieData{
        title: theMovieDbData.title.clone(),
        overview: theMovieDbData.overview.clone(),
        background_image: theMovieDbData.backdrop_path.clone(),
        card_image: theMovieDbData.poster_path.clone(),
    }
}
