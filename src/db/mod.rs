use diesel::{prelude::*, sqlite::SqliteConnection};

pub mod models;
pub mod event;
pub mod sqlite_schema;

pub fn establish_connection() -> SqliteConnection {
    let db = "./testdb.sqlite3";
    SqliteConnection::establish(db)
        .unwrap_or_else(|_| panic!("Error connecting to {}", db))
}

