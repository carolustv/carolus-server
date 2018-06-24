use diesel::sqlite::SqliteConnection;
use glob::glob;

use data::init::establish_connection;
use data::movies::get_all_movies;

use provider::{MovieData, look_up_movie};

fn update_movie_metadata(conn: &SqliteConnection) {
    let movies = get_all_movies(conn);

    for movie in movies.into_iter() {
        let movie_data = look_up_movie(movie.title, movie.year);
    }
}

pub fn load_metadata() {
    let conn = establish_connection();

    update_movie_metadata(&conn);
}
