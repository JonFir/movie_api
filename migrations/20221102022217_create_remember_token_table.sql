-- Add migration script here
CREATE TABLE remember_tokens (
   user_id TEXT NOT NULL,
   token TEXT NOT NULL,
   FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE ON UPDATE CASCADE
);