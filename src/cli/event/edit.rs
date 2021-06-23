use crate::db;
use crate::tools::cli_edit::{
    edit_line,
    edit_datetime,
    edit_option_line,
    edit_option_number,
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

    let item = db::models::event::Event::get(&conn, id).expect("Id not found");
    let mut new_item = item.clone();

    new_item.title = edit_line(&new_item.title, "Title");
    new_item.datetime = edit_datetime(&new_item.datetime, "Date & time");
    new_item.place = edit_option_line(&new_item.place, "Place");
    new_item.audience = edit_option_line(&new_item.audience, "Audience");
    new_item.status = edit_option_line(&new_item.status, "Status");
    new_item.course_id = edit_option_number(new_item.course_id, "Course ID");
    
    db::models::event::Event::update(&conn, &new_item);
}
