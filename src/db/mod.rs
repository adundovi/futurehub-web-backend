use diesel::{prelude::*, sql_types, sqlite::SqliteConnection};
use chrono::prelude::*;

pub mod models;
pub mod sqlite_schema;

use sqlite_schema::events as events;

pub fn establish_connection() -> SqliteConnection {
    let db = "./testdb.sqlite3";
    SqliteConnection::establish(db)
        .unwrap_or_else(|_| panic!("Error connecting to {}", db))
}

pub fn create_event(connection: &SqliteConnection, title: String, datetime_utc: &DateTime<Utc>) {
    let datetime = datetime_utc.naive_utc();
    let event = models::NewEvent { datetime, title, body: None, place: None, audience: None };

    diesel::insert_into(sqlite_schema::events::table)
        .values(&event)
        .execute(connection)
        .expect("Error inserting new event");
}

pub fn create_full_event(connection: &SqliteConnection, event: &models::NewEvent) {
    diesel::insert_into(sqlite_schema::events::table)
        .values(event)
        .execute(connection)
        .expect("Error inserting new event");
}

pub fn remove_event(connection: &SqliteConnection, id: i32) {
    diesel::delete(events::table.filter(events::id.eq(id)))
        .execute(connection)
        .expect(&format!("Error removing event with id = {}", id));
}

pub fn query_event(connection: &SqliteConnection) -> Vec<models::Event> {
    sqlite_schema::events::table
        .load::<models::Event>(connection)
        .expect("Error loading tasks")
}
