CREATE TABLE users (
    id INTEGER NOT NULL,
    username TEXT NOT NULL,
    email TEXT NOT NULL,
    password TEXT,
    login_session TEXT,
    oib TEXT,
    name TEXT,
    surname TEXT,
    address TEXT,
    phone TEXT,
    gender TEXT,
    birthday DATETIME,
    creation_date DATETIME NOT NULL,
    PRIMARY KEY (id)
);

CREATE TABLE login_history (
    id INTEGER NOT NULL,
    user_id INTEGER,
    login_timestamp DATETIME NOT NULL,
    PRIMARY KEY (id)
    FOREIGN KEY (user_id) REFERENCES users(id)
);
