-- Add migration script here
CREATE TABLE users(
   id TEXT NOT NULL PRIMARY KEY,
   username TEXT NOT NULL UNIQUE,
   hash TEXT NOT NULL,
   email TEXT NOT NULL UNIQUE
);