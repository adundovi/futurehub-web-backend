use crate::db;
use std::io;
use chrono::prelude::*;

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

    fn edit_line(e: &String, n: &str) -> String {
        println!("{}: {}", &n, &e);
        let mut input = String::new();
        
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                if input.trim().is_empty() {
                    println!("{}: {}", &n, e);
                    e.clone()
                } else {
                    println!("{}: {}", &n, input);
                    input.trim().to_string()
                }
            },
            Err(error) => {
                println!("error: {}", error);
                e.clone()
            }
        }
    }

    fn edit_text(e: &String, n: &str) -> String {
        let edited = edit::edit(e);
        
        match edited {
            Ok(s) => {
                println!("{}:\n{}", &n, s);
                s
            },
            Err(error) => {
                println!("error: {}", error);
                e.clone()
            }
        }
    }

    fn edit_bool(e: bool, n: &str) -> bool {
        println!("{}: {}", &n, &e);
        let mut input = String::new();
        
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                if input.trim().is_empty() {
                    println!("{}: {}", &n, e);
                    e
                } else {
                    let t = input.trim();
                    if t == "1" || t == "t" || t == "y" {
                        true
                    } else {
                        false
                    }
                }
            },
            Err(error) => {
                println!("error: {}", error);
                e
            }
        }
    }

    let conn = db::establish_connection();
    let post = db::post::get(&conn, id);
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
    db::post::update(&conn, &new_post);
}
