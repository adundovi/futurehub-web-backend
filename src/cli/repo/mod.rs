use clap::{Arg, App};
use std::collections::HashMap;

use crate::cli::menu::{Menu, Subcommand};

mod create;
mod list;
mod remove;
mod dropall;
mod edit;

pub fn menu<'a>() -> Menu<'a> {
    let mut m = Menu{
        name: "repo",
        about: "Add, modify, remove and list repository items",
        author: "Andrej Dundović <andrej.dundovic@udruga-point.hr>",
        version: "0.1",
        subcommands: HashMap::new()
    };

    let menu_create = Subcommand {
            app: App::new("new")
                .about("Add new event")
                .arg(Arg::new("TITLE")
                     .about("Title of the item")
                     .required(true)
                     .index(1)
                     )
                .arg(Arg::new("URL")
                     .about("URL of the file")
                     .required(true)
                     .index(2)
                     ),
            f: &create::f
    };
    m.push_subcommand("new", menu_create);
    
    let menu_list = Subcommand {
        app: App::new("list")
            .about("List all repository items"),
        f: &list::f
    };
    m.push_subcommand("list", menu_list);

    let menu_remove = Subcommand {
            app: App::new("remove")
                .alias("rm")
                .about("Remove post given by ID")
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
                .about("Drop all repository items from the database")
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
                .about("Edit post given by ID")
                .arg(Arg::new("ID")
                     .about("post ID")
                     .required(true)
                     .index(1)
                     ),
            f: &edit::f
    };
    m.push_subcommand("edit", menu_edit);

    m
}