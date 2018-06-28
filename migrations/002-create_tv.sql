CREATE TABLE tv_shows (
  id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  title TEXT NOT NULL,
  background_image TEXT NULL,
  card_image TEXT NULL,
  created DATETIME NOT NULL,
  updated DATETIME NOT NULL,
  CONSTRAINT unique_title_constraint UNIQUE (title)
);

CREATE TABLE tv_series (
  id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  tv_show_id INTEGER NOT NULL,
  series_number INTEGER NOT NULL,
  created DATETIME NOT NULL,
  updated DATETIME NOT NULL,
  FOREIGN KEY(tv_show_id) REFERENCES tv_shows(id),
  CONSTRAINT show_series_constraint UNIQUE (tv_show_id, series_number)
);

CREATE TABLE tv_episodes (
  id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  tv_series_id INTEGER NOT NULL,
  episode_number INTEGER NOT NULL,
  file_path TEXT NOT NULL,
  created DATETIME NOT NULL,
  updated DATETIME NOT NULL,
  FOREIGN KEY(tv_series_id) REFERENCES tv_series(id),
  CONSTRAINT series_episode_constraint UNIQUE (tv_series_id, episode_number),
  CONSTRAINT unique_file_path_constraint UNIQUE (file_path)
);
