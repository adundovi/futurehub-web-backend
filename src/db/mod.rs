use diesel::{prelude::*, sqlite::SqliteConnection};
use rocket_contrib::database;
use rocket_contrib::databases::diesel as dieseldb;

pub mod models;
pub mod event;
pub mod post;
pub mod sqlite_schema;

const DATABASE: &str = "sqlite_db";

#[database("sqlite_db")]
pub struct MainDbConn(dieseldb::SqliteConnection);

pub fn establish_connection() -> SqliteConnection {
    let c = rocket::config::RocketConfig::read().unwrap();
    let d = c.active().get_extra("databases").unwrap();
    let db = d[DATABASE]["url"].as_str().unwrap();
    SqliteConnection::establish(&db)
        .unwrap_or_else(|_| panic!("Error connecting to {}", &db))
}

