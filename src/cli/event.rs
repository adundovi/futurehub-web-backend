use super::super::db;
use super::super::tools;

use std::io;

use chrono::prelude::*;
use clap::{Arg, App};

pub fn menu<'a>() -> App<'a> {
    App::new("event")
        .about("Add, edit or list events")
        .version("0.1")
        .author("Andrej Dundović <andrej.dundovic@udruga-point.hr>")
        .subcommand(App::new("new")
                .about("Add new event")
                .arg(Arg::new("TITLE")
                     .about("Title of the event")
                     .required(true)
                     .index(1)
                     )
                )
        .subcommand(App::new("list")
                .about("List all events")
                )
        .subcommand(App::new("rm")
                .about("Remove event given by ID")
                .arg(Arg::new("ID")
                     .about("ID of the event")
                     .required(true)
                     .index(1)
                     )
                )
        .subcommand(App::new("edit")
                .about("Edit event given by ID")
                .arg(Arg::new("ID")
                     .about("ID of the event")
                     .required(true)
                     .index(1)
                     )
                )
        .subcommand(App::new("import")
                .about("Import events from CSV")
                .arg(Arg::new("FILE")
                     .about("CSV file")
                     .required(true)
                     .index(1)
                     )
                )
        .subcommand(App::new("dropall")
                .about("Drop all events from the database")
                .arg(Arg::new("yes")
                     .short('y')
                     .long("yes")
                     .about("Confirmation")
                     .required(true)
                     )
                )
}

pub fn process(args: clap::ArgMatches) -> () {
    if let Some(ref args) = args.subcommand_matches("event") {
        if let Some(ref subcommand) = args.subcommand_matches("new") {
            let title = match subcommand.value_of("TITLE") {
                Some(t) => String::from(t),
                None => String::from("Empty title"),
            };
            create_event(title);
        }
        if let Some(ref subcommand) = args.subcommand_matches("rm") {
            match subcommand.value_of("ID") {
                Some(i) => match i.parse::<i32>() {
                        Ok(i) => remove_event(i),
                        Err(_) => print!("ID should be a number"),
                    },
                None => print!("No ID given"),
            };
        }
        if let Some(ref subcommand) = args.subcommand_matches("edit") {
            match subcommand.value_of("ID") {
                Some(i) => match i.parse::<i32>() {
                        Ok(i) => edit_event(i),
                        Err(_) => print!("ID should be a number"),
                    },
                None => print!("No ID given"),
            };
        }
        if args.subcommand_matches("list").is_some() {
            list_events();
        }
        if let Some(ref subcommand) = args.subcommand_matches("import") {
            let new_events: Vec<db::models::NewEvent> = match subcommand.value_of("FILE") {
                Some(f) => tools::import::load_csv(f).unwrap(),
                None => Vec::new(),
            };
            for e in new_events.iter() {
                create_full_event(e);
            }
        }
        if let Some(ref subcommand) = args.subcommand_matches("dropall") {
            if subcommand.is_present("yes") {
                drop_all();
            }
        }
    }
}

fn create_event(title: String) {
    let conn = db::establish_connection();
    let utc_now = Utc::now();
    db::event::insert(&conn, title, &utc_now);
}

fn create_full_event(event: &db::models::NewEvent) {
    let conn = db::establish_connection();
    db::event::insert_full(&conn, event);
}

fn list_events() {
    let conn = db::establish_connection();
    for event in db::event::query(&conn) {
        println!("{}\t{}\t{}", event.id, event.title, event.datetime);
    }
}

fn remove_event(id: i32) {
    let conn = db::establish_connection();
    db::event::remove(&conn, id);
}

fn edit_event(id: i32) {

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
    let event = db::event::get(&conn, id);
    let mut new_event = event.clone();

    new_event.title = edit_line(&new_event.title, "Title");
    new_event.datetime = edit_line(&new_event.datetime.to_string(), "Date & time")
        .parse::<chrono::NaiveDateTime>().unwrap_or(new_event.datetime);
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

fn drop_all() {
    let conn = db::establish_connection();
    db::event::drop_all(&conn);
}
