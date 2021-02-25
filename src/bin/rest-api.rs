#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde;

use rocket_contrib::json::Json;
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;

use chrono::NaiveDateTime;

use futurehub_web_backend::db::models::Event;
use futurehub_web_backend::db::{query_event, establish_connection};

pub struct CORS();

impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to requests",
            kind: Kind::Response
        }
    }

    fn on_response(&self, request: &Request, response: &mut Response) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[derive(Serialize)]
pub struct EventAttribs {
    pub title: String,
    pub datetime: NaiveDateTime,
    pub body: Option<String>,
    pub audience: Option<String>,
}

#[derive(Serialize)]
pub struct EventWrapper {
    pub id: i32,
    pub r#type: String,
    pub attributes: EventAttribs,
}

#[derive(Serialize)]
struct JsonApiResponse {
    data: Vec<EventWrapper>,
}

#[get("/api")]
fn index_get() -> Json<JsonApiResponse> {
    let response = JsonApiResponse { data: vec![], };
    Json(response)
}

#[get("/api/events")]
fn events_get() -> Json<JsonApiResponse> {
    let mut response = JsonApiResponse { data: vec![], };

    let conn = establish_connection();
    for event in query_event(&conn) {
        let attribs = EventAttribs{
            title: event.title,
            body: event.body,
            datetime: event.datetime,
            audience: event.audience };
        let eventw = EventWrapper{ id: event.id, r#type: "event".to_string(), attributes: attribs };
        response.data.push(eventw);
    }

    Json(response)
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index_get])
        .mount("/", routes![events_get])
        .attach(CORS())
        .launch();
}
