use clap::{Arg, App};
use std::collections::HashMap;

use crate::cli::menu::{Menu, Subcommand};

mod attendee;
mod create;
mod list;
mod import;
mod remove;
mod dropall;
mod edit;
mod export_csv;
mod export_pdf;

pub fn menu<'a>() -> Menu<'a> {
    let mut m = Menu{
        name: "event",
        about: "Add, edit or list events",
        author: "Andrej Dundović <andrej.dundovic@udruga-point.hr>",
        version: "0.1",
        subcommands: HashMap::new()
    };

    let menu_create = Subcommand {
            app: App::new("new")
                .about("Add new event")
                .arg(Arg::new("TITLE")
                     .about("Title of the event")
                     .required(true)
                     .index(1)
                     ),
            f: &create::f
    };
    m.push_subcommand("new", menu_create);
    
    let menu_list = Subcommand {
        app: App::new("list")
            .about("List all events")
            .arg(Arg::new("date")
                 .long("date")
                 .value_name("YYYY-MM")
                 .validator(|s: &str| -> Result<(), String> {
                    match chrono::NaiveDate::parse_from_str(&format!("{}-01", s), "%Y-%m-%d") {
                        Ok(_) => Ok(()),
                        Err(_) => Err(String::from("The value is not given in the correct format YYYY-MM"))
                    }
                 })
                 .about("List events of the given date only (YYYY-MM)")
                 ),
        f: &list::f
    };
    m.push_subcommand("list", menu_list);

    let menu_remove = Subcommand {
            app: App::new("remove")
                .alias("rm")
                .about("Remove event given by ID")
                .arg(Arg::new("ID")
                     .about("ID of the event")
                     .required(true)
                     .index(1)
                     ),
            f: &remove::f
    };
    m.push_subcommand("remove", menu_remove);
    
    let menu_dropall = Subcommand {
            app: App::new("dropall")
                .about("Drop all events from the database")
                .arg(Arg::new("yes")
                     .short('y')
                     .long("yes")
                     .about("Confirmation")
                     .required(true)
                     ),
            f: &dropall::f
    };
    m.push_subcommand("dropall", menu_dropall);
    
    let menu_edit = Subcommand {
            app: App::new("edit")
                .about("Edit event given by ID")
                .arg(Arg::new("ID")
                     .about("event ID")
                     .required(true)
                     .index(1)
                     ),
            f: &edit::f
    };
    m.push_subcommand("edit", menu_edit);
        
    let menu_import = Subcommand {
            app: App::new("import")
                .about("Import events from CSV")
                .arg(Arg::new("FILE")
                     .about("CSV file")
                     .required(true)
                     .index(1)
                     ),
            f: &import::f
    };
    m.push_subcommand("import", menu_import);
    
    let menu_attendee = Subcommand {
            app: App::new("attendee")
                .about("Participant options")
                .subcommand(App::new("add")
                    .about("Add new attendee")
                    .arg(Arg::new("EID")
                         .about("Event ID")
                         .required(true)
                         .index(1)
                         )
                    .arg(Arg::new("UID")
                         .about("User ID")
                         .required(true)
                         .index(2)
                         ),
                 )
                 .subcommand(App::new("list")
                      .about("List attendees")
                      .arg(
                         Arg::new("ID")
                         .about("Event ID")
                         .required(true)
                         .index(1)
                        )
                )
                .subcommand(App::new("edit")
                      .about("edit attendees")
                      .arg(
                         Arg::new("ID")
                         .about("Event ID")
                         .required(true)
                         .index(1)
                        )
                )
                 .subcommand(App::new("export_list")
                      .about("Export attendees to PDF")
                      .arg(
                         Arg::new("ID")
                         .about("Event ID")
                         .required(true)
                         .index(1)
                        )
                )
                .subcommand(App::new("remove")
                    .about("Remove attendee")
                    .arg(Arg::new("EID")
                         .about("Event ID")
                         .required(true)
                         .index(1)
                         )
                    .arg(Arg::new("UID")
                         .about("User ID")
                         .required(true)
                         .index(2)
                         ),
                ),
            f: &attendee::f
    };
    m.push_subcommand("attendee", menu_attendee);
    
    let menu_export_csv = Subcommand {
            app: App::new("export_csv")
                .about("Export calendar of events to CSV")
                .arg(Arg::new("date")
                     .required(true)
                     .short('d')
                     .long("date")
                     .value_name("YYYY-MM")
                     .validator(|s: &str| -> Result<(), String> {
                        match chrono::NaiveDate::parse_from_str(&format!("{}-01", s), "%Y-%m-%d") {
                            Ok(_) => Ok(()),
                            Err(_) => Err(String::from("The value is not given in the correct format YYYY-MM"))
                        }
                      })
                    .about("Calendar of the given month (YYYY-MM)")
                 ),
            f: &export_csv::f
    };
    m.push_subcommand("export_csv", menu_export_csv);
    
    let menu_export_pdf = Subcommand {
            app: App::new("export_pdf")
                .about("Export calendar of events to PDF")
                .arg(Arg::new("date")
                     .required(true)
                     .short('d')
                     .long("date")
                     .value_name("YYYY-MM")
                     .validator(|s: &str| -> Result<(), String> {
                        match chrono::NaiveDate::parse_from_str(&format!("{}-01", s), "%Y-%m-%d") {
                            Ok(_) => Ok(()),
                            Err(_) => Err(String::from("The value is not given in the correct format YYYY-MM"))
                        }
                      })
                    .about("Calendar of the given month (YYYY-MM)")
                 )
                .arg(Arg::new("size")
                     .short('s')
                     .long("size")
                     .value_name("FONTSIZE")
                     .possible_values(&["smaller", "small", "normal", "large", "larger"])
                     .about("Font size of the item text")
                 )
                .arg(Arg::new("split_at")
                     .long("split_at")
                     .value_name("INT")
                     .about("Number of items in the first column")
                 ),
            f: &export_pdf::f
    };
    m.push_subcommand("export_pdf", menu_export_pdf);

    m
}
