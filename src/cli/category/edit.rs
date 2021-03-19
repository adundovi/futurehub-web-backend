use crate::db;
use crate::tools::cli_edit::{
    edit_line,
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
    let item = db::category::get(&conn, id).expect("Id not found");
    let mut new_item = item.clone();

    new_item.title = edit_line(&item.title, "Title");
    new_item.slug = edit_line(&item.slug, "Slug");
    
    match item.icon {
        Some(s) => new_item.icon = Some(edit_line(&s, "Icon")),
        None => new_item.icon = Some(edit_line(&String::new(), "Icon")),
    }

    match item.description {
        Some(s) => new_item.description = Some(edit_text(&s, "Description")),
        None => new_item.description = Some(edit_text(&String::new(), "Description")), 
    };
    
    let conn = db::establish_connection();
    db::category::update(&conn, &new_item);
}
