use crate::db;
use chrono::prelude::*;
use crate::tools::cli_edit::{
    edit_line,
    edit_number,
    edit_bool,
    edit_text,
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
    let item = db::repo_items::get(&conn, id).expect("Id not found");
    let mut new_item = item.clone();

    new_item.title = edit_line(&item.title, "Title");
    new_item.slug = edit_line(&item.slug, "Slug");
    new_item.datetime = NaiveDateTime::parse_from_str(
        &edit_line(&new_item.datetime.to_string(), "Date & time"),
        "%Y-%m-%d %H:%M:%S").unwrap_or(new_item.datetime);
    
    match item.description {
        Some(s) => new_item.description = Some(edit_text(&s, "Description")),
        None => new_item.description = Some(edit_text(&String::new(), "Description")), 
    };
    new_item.category_id = edit_number(&item.category_id, "CategoryId");
    new_item.published = edit_bool(item.published, "Published");
    
    let conn = db::establish_connection();
    db::repo_items::update(&conn, &new_item);
}
