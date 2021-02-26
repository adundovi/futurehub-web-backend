use diesel::{prelude::*, sqlite::SqliteConnection};
use chrono::prelude::*;

use super::models;
use super::sqlite_schema::events as events;

pub fn insert(connection: &SqliteConnection, title: String, datetime_utc: &DateTime<Utc>) {
    let datetime = datetime_utc.naive_utc();
    let event = models::NewEvent { datetime, title, body: None, place: None, audience: None };

    diesel::insert_into(events::table)
        .values(&event)
        .execute(connection)
        .expect("Error inserting new event");
}

pub fn insert_full(connection: &SqliteConnection, event: &models::NewEvent) {
    diesel::insert_into(events::table)
        .values(event)
        .execute(connection)
        .expect("Error inserting new event");
}

pub fn remove(connection: &SqliteConnection, id: i32) {
    diesel::delete(events::table.filter(events::id.eq(id)))
        .execute(connection)
        .expect(&format!("Error removing event with id = {}", id));
}

pub fn drop_all(connection: &SqliteConnection) {
    print!("Removing all events");
    diesel::delete(events::table)
        .execute(connection)
        .expect(&format!("Error removing all events"));
}

pub fn query(connection: &SqliteConnection) -> Vec<models::Event> {
    events::table
        .load::<models::Event>(connection)
        .expect("Error loading events")
}

pub fn get(connection: &SqliteConnection, id: i32) -> models::Event {
    events::table
        .filter(events::id.eq(id))
        .first::<models::Event>(connection)
        .expect("Error loading event")
}

pub fn update(connection: &SqliteConnection, event: &models::Event) {
    diesel::update(events::table.filter(events::id.eq(event.id)))
        .set((events::title.eq(&event.title),
              events::datetime.eq(&event.datetime),
              events::place.eq(&event.place),
              events::body.eq(&event.body),
              events::audience.eq(&event.audience),
        ))
        .execute(connection)
        .expect(&format!("Error updating event with id = {}", event.id));
}

