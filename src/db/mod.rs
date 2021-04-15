use diesel::{prelude::*, sqlite::SqliteConnection};
use rocket_contrib::database;
use rocket_contrib::databases::diesel as dieseldb;

pub mod event;
pub mod category;
pub mod models;
pub mod post;
pub mod repo_items;
pub mod sqlite_schema;
pub mod user;

const DATABASE: &str = "sqlite_db";

#[database("sqlite_db")]
pub struct MainDbConn(dieseldb::SqliteConnection);

pub fn establish_connection() -> SqliteConnection {
    let c = crate::active_config();
    let d = c.get_extra("databases").unwrap();
    let db = d[DATABASE]["url"].as_str().unwrap();
    SqliteConnection::establish(&db)
        .unwrap_or_else(|_| panic!("Error connecting to {}", &db))
}

