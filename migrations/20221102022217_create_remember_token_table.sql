-- Add migration script here
CREATE TABLE remember_tokens (
   user_id TEXT NOT NULL,
   token VARCHAR(255) NOT NULL,
   FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE ON UPDATE CASCADE
);