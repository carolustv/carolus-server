
use std::env;
use std::path::PathBuf;

use glob::glob;

use file_name::{self, ParseResult};
use data::init::establish_connection;
use data::movies::create_movie;

fn index_movie_directory(add_movie: &Fn(&PathBuf)) {
    match env::var("CAROLUS_MOVIES_PATH") {
        Ok (directories) => {
            for directory in directories.split(",") {
                for file in glob(&format!("{}/**/*.mp4", &directory)).unwrap().filter_map(Result::ok) {
                    add_movie(&file);
                }
            }
        },
        Err(_) => (),
    }
}

pub fn index() {
    let conn = establish_connection();
    index_movie_directory(&|movie_path|{
        match file_name::parse(movie_path) {
            Ok(ParseResult::Movie{ title, ..}) => {
                let file_path = movie_path.to_str().unwrap();
                info!();
                create_movie(&conn, &title, &file_path);
            },
            Err(err) => error!("Unexpected error parsing file: {}", err)
        };
    });
}
