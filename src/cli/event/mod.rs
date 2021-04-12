use clap::{Arg, App};
use std::collections::HashMap;

use crate::cli::menu::{Menu, Subcommand};

mod create;
mod list;
mod import;
mod remove;
mod dropall;
mod edit;
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
            .arg(Arg::new("month")
                 .short('m')
                 .long("month")
                 .about("List this month events only")
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
    
    let menu_export_pdf = Subcommand {
            app: App::new("export_pdf")
                .about("Export calendar of events to PDF"),
            f: &export_pdf::f
    };
    m.push_subcommand("export_pdf", menu_export_pdf);

    m
}
