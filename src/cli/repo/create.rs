use crate::db;
use chrono::prelude::*;

// create post
pub fn f(args: &clap::ArgMatches) {
    let title = match args.value_of("TITLE") {
        Some(t) => String::from(t),
        None => String::from("Empty title"),
    };
    let title = match args.value_of("URL") {
        Some(t) => String::from(t),
        None => String::from("no_url"),
    };
    let conn = db::establish_connection();
    let utc_now = Utc::now();
    db::repo::insert(&conn, title, url, &utc_now);
}
