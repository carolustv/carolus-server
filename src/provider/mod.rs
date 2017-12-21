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
    let the_movie_db_data = results.first().unwrap();
    MovieData{
        title: the_movie_db_data.title.clone(),
        overview: the_movie_db_data.overview.clone(),
        background_image: format!("https://image.tmdb.org/t/p/w1920{}", the_movie_db_data.backdrop_path.clone()),
        card_image: format!("https://image.tmdb.org/t/p/w500{}", the_movie_db_data.poster_path.clone()),
    }
}
