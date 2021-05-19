CREATE TABLE events (
    id INTEGER NOT NULL,
    title TEXT NOT NULL,
    body TEXT,
    place TEXT,
    audience TEXT,
    datetime DATETIME NOT NULL,
    status TEXT,
    course_id INTEGER;
    PRIMARY KEY (id)
    FOREIGN KEY (course_id) REFERENCES courses(id)
);
