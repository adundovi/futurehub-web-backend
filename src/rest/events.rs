use rocket_contrib::json::Json;
use rocket::http::Status;
use super::response::{Data, VectorItems, ItemWrapper, Attribs, Message, Response, ResponseWithStatus};
use crate::consts::messages;
use crate::db::{
    MainDbConn,
    models::event,
    models::event::{Event, EventAttribs},
    models::course::Course};
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
            course_id: event.course_id,
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
    
    Data::Vector(items).to_response()
}

#[get("/events")]
pub fn get(conn: MainDbConn) -> ResponseWithStatus {
    let events_course = event::Event::query(&conn);
//    let events = events_course.iter().map(|i| i.0).collect();
    response_events(events_course)
}

#[get("/events/upcoming")]
pub fn get_upcoming(conn: MainDbConn) -> ResponseWithStatus {
    let events_course = event::Event::query_upcoming(&conn, 10);
    response_events(events_course)
}

#[options("/events")]
pub fn option<'a>() -> rocket::Response<'a> {
    let mut res = rocket::Response::new();
    res.set_status(Status::new(200, "No Content"));
    res
}

#[post("/events", format = "json", data = "<event>")]
pub fn post(
    event: Json<event::NewEvent>,
    token: Result<UserToken, ResponseWithStatus>,
    conn: MainDbConn,
) -> ResponseWithStatus {
    if let Err(e) = token {
        return e;
    }
    //let t = token.unwrap();
    //TODO: group permission for this 
    event::Event::insert_full(&conn, &event);

    ResponseWithStatus {
            status: Status::Ok,
            response: Response::Message(
                Message::new(String::from(messages::MESSAGE_SENT_SUCCESS))
                )
    }
}

#[options("/events/<_id>")]
pub fn option_by_id<'a>(_id: i32) -> rocket::Response<'a> {
    let mut res = rocket::Response::new();
    res.set_status(Status::new(200, "No Content"));
    res
}

#[delete("/events/<id>")]
pub fn delete_by_id(
    conn: MainDbConn,
    token: Result<UserToken, ResponseWithStatus>,
    id: i32
) -> ResponseWithStatus {
    if let Err(e) = token {
        return e;
    }
    event::Event::remove(&conn, id);

    ResponseWithStatus {
            status: Status::Ok,
            response: Response::Message(
                Message::new(String::from(messages::MESSAGE_SENT_SUCCESS))
                )
    }
}

#[put("/events/<id>", format = "json", data = "<event>")]
pub fn put_by_id(
    conn: MainDbConn,
    token: Result<UserToken, ResponseWithStatus>,
    id: i32,
    event: Json<EventAttribs>) -> ResponseWithStatus {
    if let Err(e) = token {
        return e;
    }
    
    let item = Event::get(&conn, id).expect("Id not found");
    let mut updated_item = item.clone();
    
    updated_item.title = event.title.clone();
    updated_item.body = event.body.clone();
    updated_item.place = event.place.clone();
    updated_item.datetime = event.datetime.clone();
    updated_item.audience = event.audience.clone();
    updated_item.status = event.status.clone();
    
    Event::update(&conn, &updated_item);

    ResponseWithStatus {
            status: Status::Ok,
            response: Response::Message(
                Message::new(String::from(messages::MESSAGE_SENT_SUCCESS))
                )
    }
}
