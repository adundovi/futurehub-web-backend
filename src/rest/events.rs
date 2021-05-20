use rocket_contrib::json::Json;
use crate::db::{
    MainDbConn,
    models::event,
    models::event::Event,
    models::course::Course};
use chrono::NaiveDateTime;

#[derive(Serialize)]
pub struct EventAttribs {
    pub title: String,
    pub datetime: NaiveDateTime,
    pub body: Option<String>,
    pub place: Option<String>,
    pub audience: Option<String>,
    pub status: Option<String>,
    pub course_code: Option<String>,
}

#[derive(Serialize)]
pub struct EventWrapper {
    pub id: i32,
    pub r#type: String,
    pub attributes: EventAttribs,
}

#[derive(Serialize)]
pub struct JsonApiResponse {
    data: Vec<EventWrapper>,
}

fn response_events(events_course: Vec<(Event, Option<Course>)>) -> Json<JsonApiResponse> {
    let mut response = JsonApiResponse { data: vec![], };
    
    for (event, course) in events_course {
        let attribs = EventAttribs{
            title: event.title,
            body: event.body,
            place: event.place,
            datetime: event.datetime,
            audience: event.audience,
            status: event.status,
            course_code: {
                match course {
                    Some(c) => Some(c.code),
                    None => None,
                }
            }
        };
        let eventw = EventWrapper{ id: event.id, r#type: "event".to_string(), attributes: attribs };
        response.data.push(eventw);
    }
    Json(response)
}

#[get("/events")]
pub fn get(conn: MainDbConn) -> Json<JsonApiResponse> {
    let events_course = event::query(&conn);
//    let events = events_course.iter().map(|i| i.0).collect();
    response_events(events_course)
}

#[get("/events/upcoming")]
pub fn get_upcoming(conn: MainDbConn) -> Json<JsonApiResponse> {
    let events_course = event::query_upcoming(&conn, 10);
//    let events = events_course.iter().map(|i| i.0).collect();
    response_events(events_course)
}
