use crate::db;
use chrono::prelude::*;

// list events
pub fn f(args: &clap::ArgMatches) {
    let conn = db::establish_connection();
    if args.is_present("month") {
            let utc_now = Utc::now();
            for p in db::event::query_by_month(&conn, &utc_now) {
                println!("{}\t{}\t{}", p.id, p.title, p.datetime);
            }
    } else {
            for p in db::event::query(&conn) {
                println!("{}\t{}\t{}", p.id, p.title, p.datetime);
            }
    };
}
