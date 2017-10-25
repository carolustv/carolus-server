-- Your SQL goes here
CREATE TABLE movies (
  id INTEGER PRIMARY KEY NOT NULL,
  title TEXT NOT NULL,
  file_path TEXT NOT NULL,
  CONSTRAINT unique_file_path_constraint UNIQUE (file_path)
);
