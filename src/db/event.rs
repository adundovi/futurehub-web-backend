use diesel::{
        prelude::*,
        sqlite::SqliteConnection,
        sql_types::Timestamp,
    };
use chrono::{
        prelude::*,
        NaiveTime,
        NaiveDateTime
    };

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

pub fn get(connection: &SqliteConnection, id: i32) -> Result<models::Event, diesel::result::Error> {
    events::table
        .filter(events::id.eq(id))
        .first::<models::Event>(connection)
}

pub fn query(connection: &SqliteConnection) -> Vec<models::Event> {
    events::table
        .order(events::datetime.asc())
        .load::<models::Event>(connection)
        .expect("Error loading events")
}

fn beginning_of_month(datetime_utc: &DateTime<Utc>) -> NaiveDateTime {
    let year = datetime_utc.year();
    let month = datetime_utc.month();
    NaiveDate::from_ymd(year, month, 1).and_time(NaiveTime::from_hms(0,0,0))
}

fn end_of_month(datetime_utc: &DateTime<Utc>) -> NaiveDateTime {
    let year = datetime_utc.year();
    let month = datetime_utc.month();
    let (year, month) = if month == 12 { (year + 1, 1) } else { (year, month + 1) };
    NaiveDate::from_ymd(year, month, 1).pred().and_time(NaiveTime::from_hms(23,59,59))
}

pub fn query_by_month(connection: &SqliteConnection, datetime_utc: &DateTime<Utc>) -> Vec<models::Event> {
    let start = beginning_of_month(datetime_utc);
    let end = end_of_month(datetime_utc);
    Local::now().to_string().into_sql::<Timestamp>();
    events::table
        .filter(events::datetime.ge(start).and(events::datetime.le(end)))
        .order(events::datetime.asc())
        .load::<models::Event>(connection)
        .expect("Error loading events")
}

pub fn query_upcoming(connection: &SqliteConnection, last: i64) -> Vec<models::Event> {
    let local_time = Local::now().to_string().into_sql::<Timestamp>();
    events::table
        .filter(events::datetime.ge(local_time))
        .order(events::datetime.asc())
        .limit(last)
        .load::<models::Event>(connection)
        .expect("Error loading events")
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

