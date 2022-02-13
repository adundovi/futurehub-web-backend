extern crate rocket;
extern crate rocket_contrib;
extern crate serde;

use rocket::{
    Route,
    Request,
    Response,
    fairing::{Fairing, Info, Kind},
    http::Header,
};

use futurehub_web_backend::db;
use futurehub_web_backend::rest::{
    auth,
    courses,
    category,
    consent,
    contact,
    events,
    posts,
    profile,
    repo,
    signup,
    users,
};

// https://developer.mozilla.org/en-US/docs/Web/HTTP/CORS
// https://javascript.info/fetch-crossorigin
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
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, PUT, OPTIONS, DELETE"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

fn main() {
    let api_routes: Vec<Route> = [
                auth::get_routes(),
                category::get_routes(),
                contact::get_routes(),
                consent::get_routes(),
                courses::get_routes(),
                events::get_routes(),
                posts::get_routes(),
                profile::get_routes(),
                repo::get_routes(),
                users::get_routes(),
                signup::get_routes(),
    ].concat();
    rocket::ignite()
        .mount("/api", api_routes)
        .attach(db::MainDbConn::fairing())
        .attach(CORS())
        .launch();
}
