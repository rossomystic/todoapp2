-- Add migration script here
-- Create table todos in sqlite

CREATE TABLE todos (
  id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  user_id INTEGER NOT NULL,
  title TEXT NOT NULL,
  description TEXT NOT NULL,
  completed BOOLEAN NOT NULL,
  FOREIGN KEY(user_id) REFERENCES users(id)
);
