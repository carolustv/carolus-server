// Copyright (c) 2017 Simon Dickson
// 
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::env;

use failure::Error;
use include_dir::{Dir, File};
use rusqlite::Connection;

pub fn establish_connection() -> Result<Connection, Error> {
    let database_url = env::var("DATABASE_URL")?;
    let conn = Connection::open(database_url)?;
    migrate(&conn)?;
    Ok(conn)
}

pub fn migrate(conn: &Connection) -> Result<(), Error> {
    static MIGRATIONS_DIR: Dir = include_dir!("migrations");

    conn.execute("CREATE TABLE IF NOT EXISTS migrations (
  id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  migration_name TEXT NOT NULL,
  CONSTRAINT migration_name_constraint UNIQUE (migration_name)
);", &[])?;
    
    for file in MIGRATIONS_DIR.files {
        let file_name = file.path().file_name().and_then(|i|i.to_str()).ok_or(format_err!("not file"))?;
        let count: i32 =
            conn.query_row("SELECT COUNT(*) FROM migrations WHERE migration_name=?", &[&file_name], |row| {
                row.get(0)
            })?;
        if count != 0 {
            continue
        }
        info!("executing migration {}", file_name);
        conn.execute(file.contents_utf8().ok_or(format_err!("not file"))?, &[])?;
        conn.execute("INSERT INTO migrations (migration_name) VALUES (?1)", &[&file_name])?;
    }

    Ok(())
}


