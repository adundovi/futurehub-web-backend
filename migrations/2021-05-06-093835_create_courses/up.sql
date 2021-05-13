CREATE TABLE courses (
    id INTEGER NOT NULL,
    code TEXT NOT NULL,
    title TEXT NOT NULL,
    description TEXT,
    creation_date DATETIME NOT NULL,
    cert_template TEXT,
    lecturer TEXT,
    organizer TEXT,
    lectures INTEGER,
    lecture_duration INTEGER,
    students INTEGER,
    max_students INTEGER,
    finished BOOLEAN NOT NULL DEFAULT 'f',
    published BOOLEAN NOT NULL DEFAULT 'f',
    PRIMARY KEY (id)
);

CREATE TABLE course_users (
    id INTEGER NOT NULL,
    course_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    join_date DATETIME NOT NULL,
    leave_date DATETIME,
    score INTEGER,
    attendance INTEGER,
    note TEXT,
    PRIMARY KEY (id)
    FOREIGN KEY (user_id) REFERENCES users(id)
    FOREIGN KEY (course_id) REFERENCES courses(id)
);

CREATE TABLE course_events (
    id INTEGER NOT NULL,
    course_id INTEGER NOT NULL,
    event_id INTEGER NOT NULL,
    PRIMARY KEY (id)
    FOREIGN KEY (event_id) REFERENCES events(id)
    FOREIGN KEY (course_id) REFERENCES courses(id)
);
