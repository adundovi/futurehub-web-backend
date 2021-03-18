use crate::db;
use chrono::prelude::*;
use crate::tools::cli_edit::{
    edit_line,
};

pub fn f(args: &clap::ArgMatches) {
        match args.value_of("ID") {
            Some(i) => match i.parse::<i32>() {
                Ok(i) => edit_event(i),
                Err(_) => print!("ID should be a number"),
            },
            None => print!("No ID given"),
            };
    }

fn edit_event(id: i32) {

    let conn = db::establish_connection();
    let event = db::event::get(&conn, id).expect("Id not found");
    let mut new_event = event.clone();

    new_event.title = edit_line(&new_event.title, "Title");
    new_event.datetime = NaiveDateTime::parse_from_str(
        &edit_line(&new_event.datetime.to_string(), "Date & time"),
        "%Y-%m-%d %H:%M:%S").unwrap_or(new_event.datetime);
    
    match event.place {
        Some(s) => new_event.place = Some(edit_line(&s, "Place")),
        None => new_event.place = Some(edit_line(&String::new(), "Place")), 
    };
    match new_event.audience {
        Some(s) => new_event.audience = Some(edit_line(&s, "Audience")),
        None => new_event.audience = Some(edit_line(&String::new(), "Audience")), 
    }
    
    let conn = db::establish_connection();
    db::event::update(&conn, &new_event);
}
