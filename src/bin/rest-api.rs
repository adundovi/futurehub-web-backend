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
    let routes =
        auth::get_routes().into_iter()
        .chain(category::get_routes().into_iter())
        .chain(contact::get_routes().into_iter())
        .chain(courses::get_routes().into_iter())
        .chain(events::get_routes().into_iter())
        .chain(posts::get_routes().into_iter())
        .chain(profile::get_routes().into_iter())
        .chain(repo::get_routes().into_iter())
        .chain(users::get_routes().into_iter())
        .chain(signup::get_routes().into_iter())
        .collect::<Vec<Route>>();
    rocket::ignite()
        .mount("/api", routes)
        .attach(db::MainDbConn::fairing())
        .attach(CORS())
        .launch();
}
