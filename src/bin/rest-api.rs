#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;

use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;

use futurehub_web_backend::db;
use futurehub_web_backend::rest::events;

pub struct CORS();

impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to requests",
            kind: Kind::Response
        }
    }

    fn on_response(&self, _request: &Request, response: &mut Response) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

fn main() {
    rocket::ignite()
        .mount("/", routes![events::events_get, events::newest_events_get])
        .attach(db::MainDbConn::fairing())
        .attach(CORS())
        .launch();
}
