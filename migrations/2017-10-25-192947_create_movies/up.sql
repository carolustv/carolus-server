CREATE TABLE movies (
  id INTEGER PRIMARY KEY NOT NULL,
  title TEXT NOT NULL,
  formatted_title TEXT NOT NULL,
  file_path TEXT NOT NULL,
  created_date DATETIME NOT NULL,
  poster_path TEXT NULL,
  backdrop_path TEXT NULL,
  CONSTRAINT formatted_title_constraint UNIQUE (formatted_title),
  CONSTRAINT unique_file_path_constraint UNIQUE (file_path)
);
