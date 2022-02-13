CREATE TABLE consents (
    id INTEGER NOT NULL,
    name TEXT NOT NULL,
    surname TEXT NOT NULL,
    email TEXT NOT NULL,
    phone TEXT,
    oib TEXT NOT NULL,
    child_name TEXT NOT NULL,
    child_surname TEXT NOT NULL,
    consent_on_off TEXT NOT NULL,
    consent_type TEXT NOT NULL,
    entry_date DATETIME NOT NULL,
    verified BOOLEAN NOT NULL DEFAULT 'f',
    verify_hash TEXT,
    PRIMARY KEY (id)
);
