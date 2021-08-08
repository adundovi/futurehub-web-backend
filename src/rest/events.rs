use rocket_contrib::json::Json;
use rocket::response::status;
use rocket::http::Status;
use super::response::{Data, VectorItems, ItemWrapper, Attribs, Message, Response, ResponseWithStatus};
use crate::consts::messages;
use crate::db::{
    MainDbConn,
    models::event,
    models::event::{Event, EventAttribs},
    models::course::Course};
use chrono::NaiveDateTime;
use super::jwt::UserToken;

fn response_events(events_course: Vec<(Event, Option<Course>)>) -> ResponseWithStatus {
    let mut items = VectorItems::new();
    
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
        let w = ItemWrapper::new(event.id, "event", Attribs::EventAttribs(attribs));
        items.push(w);
    }
    
    Data::Vector(items).get_response()
}

#[get("/events")]
pub fn get(conn: MainDbConn) -> status::Custom<Json<Response>> {
    let events_course = event::Event::query(&conn);
//    let events = events_course.iter().map(|i| i.0).collect();
    let r = response_events(events_course);
    status::Custom(
        Status::from_code(r.status_code).unwrap(),
        Json(r.response),
    )
}

#[get("/events/upcoming")]
pub fn get_upcoming(conn: MainDbConn) -> status::Custom<Json<Response>> {
    let events_course = event::Event::query_upcoming(&conn, 10);
    let r = response_events(events_course);
    status::Custom(
        Status::from_code(r.status_code).unwrap(),
        Json(r.response),
    )
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
            response: Response::Message(
                Message::new(String::from(messages::MESSAGE_SENT_SUCCESS))
                )
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
            response: Response::Message(
                Message::new(String::from(messages::MESSAGE_SENT_SUCCESS))
                )
    };

    status::Custom(
        Status::from_code(response.status_code).unwrap(),
        Json(response.response),
    )
}
