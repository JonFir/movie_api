-- Add migration script here
CREATE TABLE movies(
   id INTEGER PRIMARY KEY,
   title TEXT NOT NULL,
   director TEXT NOT NULL,
   relise_date SMALLINT NOT NULL,
   rating TINYINT NOT NULL,
   poster_id uuid,
   created_at DATETIME NOT NULL
);