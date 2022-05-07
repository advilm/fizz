CREATE TABLE users (
  email         TEXT NOT NULL PRIMARY KEY,
  username      TEXT NOT NULL,
  hash          TEXT NOT NULL
);

CREATE TABLE tasks (
  id            SERIAL PRIMARY KEY,
  email         TEXT NOT NULL REFERENCES users(email),
  title         TEXT NOT NULL,
  description   TEXT NOT NULL,
  time_estimate INTERVAL NOT NULL,
  due           TIMESTAMP NOT NULL,
  completed     BOOLEAN NOT NULL,
  color         INTEGER NOT NULL
);
