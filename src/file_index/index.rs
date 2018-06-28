use std::path::Path;

use failure::Error;
use glob::glob;
use rusqlite::Connection;

use data::init::establish_connection;
use data::movies::create_movie;
use data::tv_episodes::create_tv_episode;
use file_index::parse_movie::{self, Movie};
use file_index::parse_tv::{self, Tv};

fn index_movie_directory(conn: &Connection) -> Result<(), Error> {
    match option_env!("CAROLUS_MOVIES_PATH") {
        Some (directory) => {
            let root_dir = Path::new(directory);
            for path in glob(&format!("{}/**/*.mp4", &directory))?.filter_map(Result::ok) {
                let file_path = path.to_str().ok_or(format_err!("not file"))?;
                match parse_movie::parse(&root_dir, &path) {
                    Ok(Movie{ title, ..}) => {
                        trace!("Found movie: {}, file: {}", title, file_path);
                        create_movie(&conn, &title, &file_path)?;
                    },
                    Err(err) => warn!("Could not parse movie file: {}, err: {}", file_path, err)
                }
            }
        },
        None => (),
    }
    Ok(())
}

fn index_tv_directory(conn: &Connection) -> Result<(), Error> {
    match option_env!("CAROLUS_TV_PATH") {
        Some (directory) => {
            let root_dir = Path::new(directory);
            for path in glob(&format!("{}/**/*.mp4", &directory))?.filter_map(Result::ok) {
                let file_path = path.to_str().ok_or(format_err!("not file"))?;
                match parse_tv::parse(&root_dir, &path) {
                    Ok (Tv{ title, season, episode, ..}) => {
                        trace!("Found tv episode: {}, S{}E{}, file: {}", title, season, episode, file_path);
                        create_tv_episode(&conn, title, season, episode, file_path)?;
                    },
                    Err(err) => warn!("Could not parse episode: {}, err: {}", file_path, err)
                }
            }
        },
        None => (),
    }
    Ok(())
}

pub fn index() -> Result<(), Error> {
    let conn = establish_connection()?;

    index_movie_directory(&conn)?;
    index_tv_directory(&conn)?;
    Ok(())
}
