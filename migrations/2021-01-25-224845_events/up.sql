CREATE TABLE events (
    id INTEGER NOT NULL,
    title TEXT NOT NULL,
    body TEXT,
    place TEXT,
    audience TEXT,
    datetime DATETIME NOT NULL,
    status TEXT,
    PRIMARY KEY (id)
);
