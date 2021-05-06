use crate::db;
use crate::tools::cli_edit::{
    edit_line,
    edit_number,
    edit_bool,
    edit_option_text,
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
    
    let item = db::models::repo_items::get(&conn, id).expect("Id not found");
    let mut new_item = item.clone();

    new_item.title = edit_line(&item.title, "Title");
    new_item.slug = edit_line(&item.slug, "Slug");
    new_item.datetime = edit_datetime(&item.datetime, "Date & time");
    new_item.description = edit_option_text(&new_item.description, "Description");
    new_item.category_id = edit_number(&item.category_id, "CategoryId");
    new_item.published = edit_bool(item.published, "Published");
    
    db::models::repo_items::update(&conn, &new_item);
}
