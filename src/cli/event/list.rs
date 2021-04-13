use crate::db;
use chrono::{Utc, TimeZone};

// list events
pub fn f(args: &clap::ArgMatches) {
    let conn = db::establish_connection();
    if args.is_present("date") {
            let date = Utc.datetime_from_str(
                &format!("{}-01 12:00:00", args.value_of("date").unwrap()),
                "%Y-%m-%d %H:%M:%S").unwrap().with_timezone(&Utc);
            for p in db::event::query_by_month(&conn, &date) {
                println!("{}\t{}\t{}", p.id, p.title, p.datetime);
            }
    } else {
            for p in db::event::query(&conn) {
                println!("{}\t{}\t{}", p.id, p.title, p.datetime);
            }
    };
}
