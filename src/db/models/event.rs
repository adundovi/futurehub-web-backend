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

use crate::db::sqlite_schema::events as events;
use crate::db::sqlite_schema::event_attendees as event_attendees;
use crate::db::sqlite_schema::courses as courses;
use crate::db::sqlite_schema::course_events as cevents;
use crate::db::sqlite_schema::users as users;
use crate::db::models::course::{Course, CourseEvent};
use crate::db::models::user::User;
use crate::tools::import;


#[derive(Debug, Insertable, Serialize, Deserialize, Eq, Ord, PartialEq, PartialOrd)]
#[table_name = "events"]
#[serde(rename_all = "PascalCase")]
pub struct NewEvent {
    #[serde(with= "import::date_serializer")]
    pub datetime: NaiveDateTime,
    pub title: String,
    pub body: Option<String>,
    pub place: Option<String>,
    pub audience: Option<String>,
    pub status: Option<String>,
}

#[derive(Queryable, Serialize, Deserialize, Clone)]
pub struct Event {
    pub id: i32,
    pub title: String,
    pub body: Option<String>,
    pub place: Option<String>,
    pub audience: Option<String>,
    pub datetime: NaiveDateTime, // UTC
    pub status: Option<String>,
}

#[derive(Debug, Insertable, Serialize, Deserialize, Eq, Ord, PartialEq, PartialOrd)]
#[table_name = "event_attendees"]
#[serde(rename_all = "PascalCase")]
pub struct NewEventAttendee {
    pub event_id: i32,
    pub user_id: i32,
    pub join_datetime: Option<NaiveDateTime>,
    pub leave_datetime: Option<NaiveDateTime>,
    pub presence: Option<String>,
    pub note: Option<String>,
}

#[derive(Queryable, Serialize, Deserialize, Clone)]
pub struct EventAttendee {
    pub id: i32,
    pub event_id: i32,
    pub user_id: i32,
    pub join_datetime: Option<NaiveDateTime>,
    pub leave_datetime: Option<NaiveDateTime>,
    pub presence: Option<String>,
    pub note: Option<String>,
}


pub fn insert(conn: &SqliteConnection, title: String, datetime_utc: &DateTime<Utc>) {
    let datetime = datetime_utc.naive_utc();
    let event = NewEvent { datetime, title, body: None, place: None, audience: None, status: None };

    diesel::insert_into(events::table)
        .values(&event)
        .execute(conn)
        .expect("Error inserting new event");
}

pub fn insert_full(conn: &SqliteConnection, event: &NewEvent) {
    diesel::insert_into(events::table)
        .values(event)
        .execute(conn)
        .expect("Error inserting new event");
}

pub fn remove(conn: &SqliteConnection, id: i32) {
    diesel::delete(events::table.filter(events::id.eq(id)))
        .execute(conn)
        .expect(&format!("Error removing event with id = {}", id));
}

pub fn drop_all(conn: &SqliteConnection) {
    print!("Removing all events");
    diesel::delete(events::table)
        .execute(conn)
        .expect(&format!("Error removing all events"));
}

pub fn get(conn: &SqliteConnection, id: i32) -> Result<Event, diesel::result::Error> {
    events::table
        .filter(events::id.eq(id))
        .first::<Event>(conn)
}

pub fn query(conn: &SqliteConnection) -> Vec<Event> {
    events::table
        .order(events::datetime.asc())
        .load::<Event>(conn)
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

pub fn query_by_month(conn: &SqliteConnection, datetime_utc: &DateTime<Utc>) -> Vec<Event> {
    let start = beginning_of_month(datetime_utc);
    let end = end_of_month(datetime_utc);
    Local::now().to_string().into_sql::<Timestamp>();
    events::table
        .filter(events::datetime.ge(start).and(events::datetime.le(end)))
        .order(events::datetime.asc())
        .load::<Event>(conn)
        .expect("Error loading events")
}

pub fn query_with_course_by_month(conn: &SqliteConnection, datetime_utc: &DateTime<Utc>) -> Vec<(Event, (CourseEvent, Course))> {
    let start = beginning_of_month(datetime_utc);
    let end = end_of_month(datetime_utc);
    Local::now().to_string().into_sql::<Timestamp>();
    
    events::table
        .inner_join(cevents::table.inner_join(courses::table))
        .filter(events::datetime.ge(start).and(events::datetime.le(end)))
        .order(events::datetime.asc())
        .load::<(Event, (CourseEvent, Course))>(conn)
        .expect("Error loading data")
}

pub fn query_upcoming(conn: &SqliteConnection, last: i64) -> Vec<Event> {
    let local_time = Local::now().to_string().into_sql::<Timestamp>();
    events::table
        .filter(events::datetime.ge(local_time))
        .order(events::datetime.asc())
        .limit(last)
        .load::<Event>(conn)
        .expect("Error loading events")
}

pub fn update(conn: &SqliteConnection, event: &Event) {
    diesel::update(events::table.filter(events::id.eq(event.id)))
        .set((events::title.eq(&event.title),
              events::datetime.eq(&event.datetime),
              events::place.eq(&event.place),
              events::body.eq(&event.body),
              events::audience.eq(&event.audience),
              events::status.eq(&event.status),
        ))
        .execute(conn)
        .expect(&format!("Error updating event with id = {}", event.id));
}

pub fn add_attendee(event_id_: i32,
                    user_id_: i32,
                    conn: &SqliteConnection) -> bool {
        
        let relation = NewEventAttendee {
            event_id: event_id_,
            user_id: user_id_,
            join_datetime: Some(Utc::now().naive_utc()),
            leave_datetime: None,
            presence: None,
            note: None,
        };
        diesel::insert_into(event_attendees::table)
            .values(&relation)
            .execute(conn)
            .is_ok()
}
   
pub fn list_attendees(event_id_: i32, conn: &SqliteConnection) -> Vec<(User, EventAttendee)> {
        users::table
            .inner_join(event_attendees::table)
            .filter(event_attendees::event_id.eq(event_id_))
            .load::<(User, EventAttendee)>(conn)
            .expect("Error loading users as attendees")
}

pub fn remove_attendee(event_id_: i32,
                       user_id_: i32,
                       conn: &SqliteConnection) {
        
        diesel::delete(event_attendees::table.filter(event_attendees::event_id.eq(event_id_)
                                            .and(event_attendees::user_id.eq(user_id_))))
            .execute(conn)
            .expect(&format!("Error relation user-course event_id = {}, user_id = {}",
                             event_id_, user_id_));
}
