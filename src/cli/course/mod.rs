use clap::{Arg, App};
use std::collections::HashMap;

use crate::cli::menu::{Menu, Subcommand};

mod certform;
mod certs;
mod create;
mod list;
mod remove;
mod dropall;
mod edit;
mod event;
mod participant;

pub fn menu<'a>() -> Menu<'a> {
    let mut m = Menu{
        name: "course",
        about: "Add, modify, remove and list courses",
        author: "Andrej Dundović <andrej.dundovic@udruga-point.hr>",
        version: "0.1",
        subcommands: HashMap::new()
    };

    let menu_create = Subcommand {
            app: App::new("new")
                .about("Add new course")
                .arg(Arg::new("TITLE")
                     .about("Title of the new course")
                     .required(true)
                     .index(1)
                     )
                .arg(Arg::new("CODE")
                     .about("Code of the new course")
                     .required(true)
                     .index(2)
                     ),
            f: &create::f
    };
    m.push_subcommand("new", menu_create);
    
    let menu_list = Subcommand {
        app: App::new("list")
            .about("List all courses"),
        f: &list::f
    };
    m.push_subcommand("list", menu_list);

    let menu_remove = Subcommand {
            app: App::new("remove")
                .alias("rm")
                .about("Remove the course given by ID")
                .arg(Arg::new("ID")
                     .about("Course ID")
                     .required(true)
                     .index(1)
                     ),
            f: &remove::f
    };
    m.push_subcommand("remove", menu_remove);
    
    let menu_dropall = Subcommand {
            app: App::new("dropall")
                .about("Drop all courses from the database")
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
                .about("Edit course given by ID")
                .arg(Arg::new("ID")
                     .about("Course ID")
                     .required(true)
                     .index(1)
                     ),
            f: &edit::f
    };
    m.push_subcommand("edit", menu_edit);
    
    let menu_participant = Subcommand {
            app: App::new("participant")
                .about("Participant options")
                .subcommand(App::new("add")
                    .about("Add new participant")
                    .arg(Arg::new("CID")
                         .about("Course ID")
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
                      .about("List participants")
                      .arg(
                         Arg::new("ID")
                         .about("Course ID")
                         .required(true)
                         .index(1)
                        )
                )
                .subcommand(App::new("remove")
                    .about("Remove participant")
                    .arg(Arg::new("CID")
                         .about("Course ID")
                         .required(true)
                         .index(1)
                         )
                    .arg(Arg::new("UID")
                         .about("User ID")
                         .required(true)
                         .index(2)
                         ),
                ),
            f: &participant::f
    };
    m.push_subcommand("participant", menu_participant);
    
    let menu_event = Subcommand {
            app: App::new("event")
                .about("Event options")
                .subcommand(App::new("add")
                    .about("Add event to the course")
                    .arg(Arg::new("CID")
                         .about("Course ID")
                         .required(true)
                         .index(1)
                         )
                    .arg(Arg::new("EID")
                         .about("Event ID")
                         .required(true)
                         .index(2)
                         ),
                 )
                 .subcommand(App::new("list")
                      .about("List events in the course")
                      .arg(
                         Arg::new("CID")
                         .about("Course ID")
                         .required(true)
                         .index(1)
                        )
                )
                .subcommand(App::new("remove")
                    .about("Remove event from course")
                    .arg(Arg::new("CID")
                         .about("Course ID")
                         .required(true)
                         .index(1)
                         )
                    .arg(Arg::new("EID")
                         .about("Event ID")
                         .required(true)
                         .index(2)
                         ),
                ),
            f: &event::f
    };
    m.push_subcommand("event", menu_event);
    
    let menu_certs = Subcommand {
            app: App::new("certs")
                .about("Generate certificates")
                .arg(Arg::new("ID")
                     .about("Course ID")
                     .required(true)
                     .index(1)
                     ),
            f: &certs::f
    };
    m.push_subcommand("certs", menu_certs);
    
    let menu_cert_form = Subcommand {
            app: App::new("cert_form")
                .about("Generate certificates form")
                .arg(Arg::new("ID")
                     .about("Course ID")
                     .required(true)
                     .index(1)
                     ),
            f: &certform::f
    };
    m.push_subcommand("cert_form", menu_cert_form);
    
    m
}
