-- Add migration script here
CREATE TABLE remember_tokens (
   user_id CHAR(36),
   token VARCHAR(255) NOT NULL,
   FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE ON UPDATE CASCADE
);