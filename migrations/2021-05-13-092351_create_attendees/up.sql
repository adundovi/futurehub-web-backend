CREATE TABLE event_attendees (
    id INTEGER NOT NULL,
    event_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    join_datetime DATETIME,
    leave_datetime DATETIME,
    presence TEXT,
    note TEXT,
    PRIMARY KEY (id)
    FOREIGN KEY (user_id) REFERENCES users(id)
    FOREIGN KEY (event_id) REFERENCES courses(id)
);

