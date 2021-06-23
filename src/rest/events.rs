use rocket_contrib::json::Json;
use rocket::response::status;
use rocket::http::Status;
use super::response::{Response, ResponseWithStatus};
use crate::consts::messages;
use crate::db::{
    MainDbConn,
    models::event,
    models::event::Event,
    models::course::Course};
use chrono::NaiveDateTime;
use super::jwt::UserToken;

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
    let events_course = event::Event::query(&conn);
//    let events = events_course.iter().map(|i| i.0).collect();
    response_events(events_course)
}

#[get("/events/upcoming")]
pub fn get_upcoming(conn: MainDbConn) -> Json<JsonApiResponse> {
    let events_course = event::Event::query_upcoming(&conn, 10);
//    let events = events_course.iter().map(|i| i.0).collect();
    response_events(events_course)
}

#[options("/events")]
pub fn option_event<'a>() -> rocket::Response<'a> {
    let mut res = rocket::Response::new();
    res.set_status(Status::new(200, "No Content"));
    res
}

#[post("/events", format = "json", data = "<event>")]
pub fn post_event(
    event: Json<event::NewEvent>,
    token: Result<UserToken, status::Custom<Json<Response>>>,
    conn: MainDbConn,
) -> status::Custom<Json<Response>> {
    if let Err(e) = token {
        return e;
    }
    //let t = token.unwrap();
    //TODO: group permission for this 
    event::Event::insert_full(&conn, &event);

    let response = ResponseWithStatus {
            status_code: Status::Ok.code,
            response: Response {
                message: String::from(messages::MESSAGE_SENT_SUCCESS),
                data: serde_json::to_value("").unwrap(),
            },
    };

    status::Custom(
        Status::from_code(response.status_code).unwrap(),
        Json(response.response),
    )
}

#[options("/events/<_id>")]
pub fn option_event_id<'a>(_id: i32) -> rocket::Response<'a> {
    let mut res = rocket::Response::new();
    res.set_status(Status::new(200, "No Content"));
    res
}

#[delete("/events/<id>")]
pub fn delete_event(
    conn: MainDbConn,
    token: Result<UserToken, status::Custom<Json<Response>>>,
    id: i32
) -> status::Custom<Json<Response>> {
    if let Err(e) = token {
        return e;
    }
    event::Event::remove(&conn, id);

    let response = ResponseWithStatus {
            status_code: Status::Ok.code,
            response: Response {
                message: String::from(messages::MESSAGE_SENT_SUCCESS),
                data: serde_json::to_value("").unwrap(),
            },
    };

    status::Custom(
        Status::from_code(response.status_code).unwrap(),
        Json(response.response),
    )
}
