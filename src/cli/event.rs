use super::super::db;
use super::super::tools;

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
        .subcommand(App::new("import")
                .about("Import events from CSV")
                .arg(Arg::new("FILE")
                     .about("CSV file")
                     .required(true)
                     .index(1)
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
        if let Some(ref _subcommand) = args.subcommand_matches("list") {
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
    }
}

fn create_event(title: String) {
    let conn = db::establish_connection();
    let utc_now = Utc::now();
    db::create_event(&conn, title, &utc_now);
}

fn create_full_event(event: &db::models::NewEvent) {
    let conn = db::establish_connection();
    db::create_full_event(&conn, event);
}

fn list_events() {
    let conn = db::establish_connection();
    for event in db::query_event(&conn) {
        println!("{}\t{}\t{}", event.id, event.title, event.datetime);
    }
}

fn remove_event(id: i32) {
    let conn = db::establish_connection();
    db::remove_event(&conn, id);
}
