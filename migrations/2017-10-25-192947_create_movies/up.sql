CREATE TABLE movies (
  id INTEGER PRIMARY KEY NOT NULL,
  title TEXT NOT NULL,
  file_path TEXT NOT NULL,
  background_image TEXT NULL,
  card_image TEXT NULL,
  created DATETIME NOT NULL,
  updated DATETIME NOT NULL,
  CONSTRAINT unique_file_path_constraint UNIQUE (file_path)
);
