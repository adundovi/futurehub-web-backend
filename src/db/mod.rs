use diesel::{prelude::*, sqlite::SqliteConnection};

pub mod models;
pub mod event;
pub mod sqlite_schema;

pub fn establish_connection() -> SqliteConnection {
    let c = rocket::config::RocketConfig::read().unwrap();
    let d = c.active().get_extra("databases").unwrap();
    let db = d["sqlite_db"]["url"].as_str().unwrap();
    SqliteConnection::establish(&db)
        .unwrap_or_else(|_| panic!("Error connecting to {}", &db))
}

