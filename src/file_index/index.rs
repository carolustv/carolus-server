use diesel::sqlite::SqliteConnection;
use glob::glob;

use data::init::establish_connection;
use data::movies::create_movie;
use data::tv_shows::create_tv_show;
use data::tv_series::create_tv_series;
use data::tv_episodes::create_tv_episode;

use file_index::parse_movie::{self, Movie,};
use file_index::parse_tv::{self, Tv};

static VIDEO_EXTENSIONS: &'static [&'static str] = &["mp4", "mkv", "m4v"];

fn has_video_extension(file_path: &str) -> bool {
    VIDEO_EXTENSIONS.iter().any(|ext|file_path.ends_with(ext))
}

fn index_movie_directory(conn: &SqliteConnection) {
    match option_env!("CAROLUS_MOVIES_PATH") {
        Some (directory) => {
            for path in glob(&format!("{}/**/*", &directory)).unwrap().filter_map(Result::ok) {
                let file_path = path.to_str().unwrap();
                if has_video_extension(&file_path) {
                    match parse_movie::parse(&file_path) {
                        Ok(Movie{ title, ..}) => {
                            create_movie(&conn, &title, &format_title(&title), &file_path).unwrap();
                        },
                        Err(err) => info!("Could not parse movie file: {}, err: {}", file_path, err)
                    }
                }
            }
        },
        None => (),
    }
}

fn index_tv_directory(conn: &SqliteConnection) {
    match option_env!("CAROLUS_TV_PATH") {
        Some (directory) => {
            for path in glob(&format!("{}/**/*{{.mp4,*.mkv}}", &directory)).unwrap().filter_map(Result::ok) {
                let file_path = path.to_str().unwrap();
                match parse_tv::parse(&file_path) {
                    Ok (Tv{ title, season, episode, ..}) => {
                        let show = create_tv_show(&conn, &title);
                        let series = create_tv_series(&conn, show.id, season);
                        create_tv_episode(&conn, series.id, episode, file_path);
                    },
                    Err(err) => info!("Could not parse episode: {}, err: {}", file_path, err)
                }
            }
        },
        None => (),
    }
}

pub fn index() {
    let conn = establish_connection();

    index_movie_directory(&conn);
    index_tv_directory(&conn);
}

pub fn format_title(title: &str) -> String {
    title.replace(" ", "-").to_lowercase()
}

#[test]
fn american_history_x(){
    assert_eq!("american-history-x", format_title("American History X"));
}

#[test]
fn great_escape(){
    assert_eq!("great-escape", format_title("Great Escape"));
}

#[test]
fn die_hard(){
    assert_eq!("die-hard", format_title("Die Hard"));
}
