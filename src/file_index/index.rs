use diesel::sqlite::SqliteConnection;
use glob::glob;

use data::init::establish_connection;
use data::movies::create_movie;

use file_index::file_name::{self, Movie};

fn index_movie_directory(conn: &SqliteConnection) {
    match option_env!("CAROLUS_MOVIES_PATH") {
        Some (directory) => {
            for movie_path in glob(&format!("{}/**/*{{.mp4,*.mkv}}", &directory)).unwrap().filter_map(Result::ok) {
                match file_name::parse_movie(&movie_path) {
                    Ok(Movie{ title, ..}) => {
                        let file_path = movie_path.to_str().unwrap();
                        create_movie(&conn, &title, &file_path);
                    },
                    Err(err) => info!("Could not parse movie file: {}, err: {}", movie_path.display(), err)
                }
            }
        },
        None => (),
    }
}

//fn index_tv_directory(conn: &SqliteConnection) {
//    match option_env!("CAROLUS_TV_PATH") {
//        Some (directory) => {
//            for tv_path in glob(&format!("{}/**/*{{.mp4,*.mkv}}", &directory)).unwrap().filter_map(Result::ok) {
//                match file_name::parse_tv(&tv_path) {
//                    Some (Tv{ title, ..}) => {
//                        let file_path = tv_path.to_str().unwrap();
//                        create_movie(&conn, &title, &file_path);
//                    },
//                    None => info!("Could not parse episode: {}", tv_path.display())
//                }
//            }
//        },
//        None => (),
//    }
//}

pub fn index() {
    let conn = establish_connection();

    index_movie_directory(&conn);
    //index_tv_directory(&conn);
}
