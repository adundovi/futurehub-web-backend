CREATE TABLE categories (
    id INTEGER NOT NULL,
    title TEXT NOT NULL,
    slug TEXT NOT NULL,
    icon TEXT,
    description TEXT,
    PRIMARY KEY (id)
);

CREATE TABLE repo_items (
  id INTEGER NOT NULL,
  title VARCHAR NOT NULL,
  slug VARCHAR NOT NULL,
  filepath VARCHAR NOT NULL,
  description TEXT,
  category_id INTEGER NOT NULL,
  filetype VARCHAR,
  filehash VARCHAR,
  filesize INTEGER,
  published BOOLEAN NOT NULL DEFAULT 'f',
  datetime DATETIME NOT NULL,
  PRIMARY KEY (id),
  FOREIGN KEY (category_id) REFERENCES categories(id)
)

