use super::super::db;
use super::super::tools;
use super::menu::{Menu, Subcommand};

use std::io;
use std::collections::HashMap;

use chrono::prelude::*;
use clap::{Arg, App};

pub fn menu<'a>() -> Menu<'a> {

    let mut m = Menu{
        name: "post",
        about: "Add, modify, remove and list posts",
        author: "Andrej Dundović <andrej.dundovic@udruga-point.hr>",
        version: "0.1",
        subcommands: HashMap::new()
    };

    // create post
    fn f_create(args: &clap::ArgMatches) {
        let title = match args.value_of("TITLE") {
                Some(t) => String::from(t),
                None => String::from("Empty title"),
        };
        let conn = db::establish_connection();
        let utc_now = Utc::now();
        db::post::insert(&conn, title, &utc_now);
    }
    let menu_create = Subcommand {
            app: App::new("new")
                .about("Add new event")
                .arg(Arg::new("TITLE")
                     .about("Title of the event")
                     .required(true)
                     .index(1)
                     ),
            f: &f_create
    };
    m.push_subcommand("new", menu_create);
    
    // list posts
    fn f_list(_args: &clap::ArgMatches) {
        let conn = db::establish_connection();
        for p in db::post::query(&conn) {
            println!("{}\t{}\t{}", p.id, p.title, p.datetime);
        }
    }
    let menu_list = Subcommand {
        app: App::new("list")
            .about("List all posts"),
        f: &f_list
    };
    m.push_subcommand("list", menu_list);

    // rm post
    fn f_remove(args: &clap::ArgMatches) {
        match args.value_of("ID") {
            Some(id) => match id.parse::<i32>() {
                Ok(id) => {
                    let conn = db::establish_connection();
                    db::post::remove(&conn, id);
                    },
                Err(_) => print!("ID should be a number"),
            },
            None => print!("No ID given"),
        };
    }
    let menu_remove = Subcommand {
            app: App::new("remove")
                .alias("rm")
                .about("Remove post given by ID")
                .arg(Arg::new("ID")
                     .about("ID of the event")
                     .required(true)
                     .index(1)
                     ),
            f: &f_remove
    };
    m.push_subcommand("remove", menu_remove);
    
    // dropall post
    fn f_dropall(args: &clap::ArgMatches) {
        if args.is_present("yes") {
            let conn = db::establish_connection();
            db::post::drop_all(&conn);
        } else {
            print!("Please confirm the action with --yes");
        }
    }
    let menu_dropall = Subcommand {
            app: App::new("dropall")
                .about("Drop all posts from the database")
                .arg(Arg::new("yes")
                     .short('y')
                     .long("yes")
                     .about("Confirmation")
                     .required(true)
                     ),
            f: &f_dropall
    };
    m.push_subcommand("dropall", menu_dropall);

/*
        if let Some(ref subcommand) = args.subcommand_matches("edit") {
            match subcommand.value_of("ID") {
                Some(i) => match i.parse::<i32>() {
                        Ok(i) => edit_post(i),
                        Err(_) => print!("ID should be a number"),
                    },
                None => print!("No ID given"),
            };
        }
*/
    /*
    m.push_subcommand(
        App::new("edit")
                .about("Edit event given by ID")
                .arg(Arg::new("ID")
                     .about("ID of the event")
                     .required(true)
                     .index(1)
                     ));*/
    m
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

    let conn = db::establish_connection();
    let post = db::post::get(&conn, id);
    let mut new_post = post.clone();

    new_post.title = edit_line(&new_post.title, "Title");
    new_post.datetime = NaiveDateTime::parse_from_str(
        &edit_line(&new_post.datetime.to_string(), "Date & time"),
        "%Y-%m-%d %H:%M:%S").unwrap_or(new_post.datetime);
    /*
    match post.place {
        Some(s) => new_post.place = Some(edit_line(&s, "Place")),
        None => new_post.place = Some(edit_line(&String::new(), "Place")), 
    };
    match new_post.audience {
        Some(s) => new_post.audience = Some(edit_line(&s, "Audience")),
        None => new_post.audience = Some(edit_line(&String::new(), "Audience")), 
    }
    */
    let conn = db::establish_connection();
    db::post::update(&conn, &new_post);
}
