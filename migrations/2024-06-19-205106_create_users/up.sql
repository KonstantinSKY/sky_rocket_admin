-- Your SQL goes here
CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  username varchar(64) NOT NULL UNIQUE,
  password varchar(128) NOT NULL,
  email varchar(128) NOT NULL UNIQUE,
  first_name varchar(256),
  last_name varchar(256),
  is_active boolean DEFAULT true NOT NULL,
  is_staff boolean DEFAULT false NOT NULL,
  is_superuser boolean DEFAULT false NOT NULL,
  last_login TIMESTAMP,
  created_at TIMESTAMP DEFAULT NOW() NOT NULL
)

