use crate::db;
use crate::tools::export::save_csv;
//use std::path::Path;
use chrono::prelude::*;
use std::error::Error;

pub fn f(args: &clap::ArgMatches) {
        
    let date = Utc.datetime_from_str(
                   &format!("{}-01 12:00:00", args.value_of("date").unwrap()),
                   "%Y-%m-%d %H:%M:%S").unwrap().with_timezone(&Utc);
    
    /*let output_path_str = format!("./tmp/calendar-{year}-{month}.csv",
                           year = date.year(),
                           month = date.month());
    let output_path = Path::new(&output_path_str);
    */
    let events = load_calendar(date);
    match events {
        Ok(events) => save_csv(events).unwrap(),
        Err(_) => println!("Something went wrong :-p"),
    }
}

fn load_calendar(dt: DateTime<Utc>) -> Result<Vec<db::models::event::Event>, Box<dyn Error>> {
    let conn = db::establish_connection();
    let events_course: Vec<(db::models::event::Event, _)> = db::models::event::query_by_month(&conn, &dt);
    let events = events_course.into_iter().map(|i| i.0).collect();
    Ok(events)
}
