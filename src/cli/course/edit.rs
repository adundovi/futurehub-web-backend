use crate::db;
use crate::db::model_traits::Queries;
use crate::tools::cli_edit::{
    edit_bool,
    edit_line,
    edit_option_line,
    edit_option_text,
    edit_option_number,
    edit_datetime,
};

pub fn f(args: &clap::ArgMatches) {
        match args.value_of("ID") {
            Some(i) => match i.parse::<i32>() {
                Ok(i) => edit_item(i),
                Err(_) => print!("ID should be a number"),
            },
            None => print!("No ID given"),
            };
    }

fn edit_item(id: i32) {

    let conn = db::establish_connection();
    
    let item = db::models::course::Course::get(&conn, id).expect("Id not found");
    let mut new_item = item.clone();

    new_item.code = edit_line(&item.code, "Code");
    new_item.title = edit_line(&item.title, "Title");
    new_item.description = edit_option_text(&item.description, "Description");
    new_item.creation_date = edit_datetime(&item.creation_date, "Creation date");
    new_item.cert_template = edit_option_line(&item.cert_template, "Certificate template");
    new_item.lecturer = edit_option_line(&item.lecturer, "Lecturer");
    new_item.organizer = edit_option_line(&item.organizer, "Organizer");
    new_item.lectures = edit_option_number(item.lectures, "Lectures");
    new_item.students = edit_option_number(item.students, "Students");
    new_item.max_students = edit_option_number(item.max_students, "Max. students");
    new_item.finished = edit_bool(item.finished, "Finished");
    new_item.published = edit_bool(item.published, "Published");
            
    db::models::course::Course::update(&new_item, &conn);
}
