CREATE TABLE users (
  id            UUID PRIMARY KEY,
  username      TEXT NOT NULL,
  hash          TEXT NOT NULL
);

CREATE TABLE tasks (
  id            SERIAL PRIMARY KEY,
  user_id       UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  title         TEXT NOT NULL,
  description   TEXT NOT NULL,
  priority      INTEGER NOT NULL,
  time_estimate INTEGER NOT NULL,
  due           BIGINT NOT NULL,
  recurring     SMALLINT NOT NULL,
  completed     BOOLEAN NOT NULL,
  color         INTEGER NOT NULL
);
