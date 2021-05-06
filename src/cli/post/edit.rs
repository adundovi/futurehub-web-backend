use crate::db;
use chrono::prelude::*;
use crate::tools::cli_edit::{
    edit_line,
    edit_bool,
    edit_text,
};

pub fn f(args: &clap::ArgMatches) {
        match args.value_of("ID") {
            Some(i) => match i.parse::<i32>() {
                Ok(i) => edit_post(i),
                Err(_) => print!("ID should be a number"),
            },
            None => print!("No ID given"),
            };
    }

fn edit_post(id: i32) {

    let conn = db::establish_connection();
    let post = db::models::post::get(&conn, id).expect("Id not found");
    let mut new_post = post.clone();

    new_post.title = edit_line(&new_post.title, "Title");
    new_post.slug = edit_line(&new_post.slug, "Slug");
    new_post.datetime = NaiveDateTime::parse_from_str(
        &edit_line(&new_post.datetime.to_string(), "Date & time"),
        "%Y-%m-%d %H:%M:%S").unwrap_or(new_post.datetime);
    
    match post.body {
        Some(s) => new_post.body = Some(edit_text(&s, "Place")),
        None => new_post.body = Some(edit_text(&String::new(), "Place")), 
    };
    
    new_post.published = edit_bool(post.published, "Published");
    
    let conn = db::establish_connection();
    db::models::post::update(&conn, &new_post);
}
