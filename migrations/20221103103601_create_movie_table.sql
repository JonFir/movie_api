-- Add migration script here
CREATE TABLE movies(
   id uuid NOT NULL PRIMARY KEY,
   title TEXT NOT NULL,
   director TEXT NOT NULL,
   relise_date SMALLINT NOT NULL,
   rating TINYINT NOT NULL,
   poster_id uuid
);