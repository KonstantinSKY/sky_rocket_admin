-- Your SQL goes here
CREATE TABLE groups (
  id SERIAL PRIMARY KEY,
  name varchar(128) NOT NULL UNIQUE,
  description text,
  created_at TIMESTAMP DEFAULT NOW() NOT NULL
)
