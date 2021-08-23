#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;

use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;

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
    rocket::ignite()
        .mount("/api", routes![
               auth::ping,
               auth::option_ping,
               auth::post_login,
               auth::option_login,
               auth::post_signup,
               events::get,
               events::get_upcoming,
               events::post,
               events::option,
               events::delete_by_id,
               events::option_by_id,
               events::put_by_id,
               posts::get,
               posts::get_by_id,
               posts::get_by_slug,
               repo::get,
               repo::get_by_id,
               repo::get_by_slug,
               repo::get_stream_by_slug,
               courses::get,
               courses::get_by_id,
               courses::get_by_code,
               courses::get_by_id_events,
               courses::get_by_id_participants,
               courses::option,
               courses::option_by_id_events,
               courses::option_by_id_participants,
               courses::post,
               courses::option_by_id,
               courses::delete_by_id,
               courses::put_by_id,
               category::get,
               category::get_by_id,
               category::get_by_slug,
               contact::post_form,
               profile::get_info,
               profile::option_info,
               users::get,
               users::option,
               users::post,
               users::option_by_id,
               users::delete_by_id,
               users::put_by_id,
               signup::post,
               signup::option,
        ])
        .attach(db::MainDbConn::fairing())
        .attach(CORS())
        .launch();
}
