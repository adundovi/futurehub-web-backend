use crate::db;
use chrono::prelude::*;

// create post
pub fn f(args: &clap::ArgMatches) {
    let title = match args.value_of("TITLE") {
        Some(t) => String::from(t),
        None => String::from("Empty title"),
    };
    let conn = db::establish_connection();
    let utc_now = Utc::now();
    db::models::post::insert(&conn, title, &utc_now);
}
