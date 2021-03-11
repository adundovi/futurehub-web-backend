-- Your SQL goes here
CREATE TABLE repo (
  id INTEGER NOT NULL,
  title VARCHAR NOT NULL,
  slug VARCHAR NOT NULL,
  filepath VARCHAR NOT NULL,
  description TEXT,
  category VARCHAR,
  filetype VARCHAR,
  published BOOLEAN NOT NULL DEFAULT 'f',
  datetime DATETIME NOT NULL,
  PRIMARY KEY (id)
)
