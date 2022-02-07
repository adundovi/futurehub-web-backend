use clap::{Arg, App};
use std::collections::HashMap;

use crate::cli::menu::{Menu, Subcommand};

mod create;
mod dropall;
mod edit;
mod export_csv;
mod import;
mod list;
mod password;
mod remove;

pub fn menu<'a>() -> Menu<'a> {
    let mut m = Menu{
        name: "user",
        about: "Add, modify, remove and list users",
        author: "Andrej Dundović <andrej.dundovic@udruga-point.hr>",
        version: "0.1",
        subcommands: HashMap::new()
    };

    let menu_create = Subcommand {
            app: App::new("new")
                .about("Add new user")
                .arg(Arg::new("USERNAME")
                     .help("Username of the new user")
                     .required(true)
                     .index(1)
                     )
                .arg(Arg::new("EMAIL")
                     .help("Email of the new user")
                     .required(true)
                     .index(2)
                     ),
            f: &create::f
    };
    m.push_subcommand("new", menu_create);
    
    let menu_list = Subcommand {
        app: App::new("list")
            .about("List all users"),
        f: &list::f
    };
    m.push_subcommand("list", menu_list);

    let menu_remove = Subcommand {
            app: App::new("remove")
                .alias("rm")
                .about("Remove the user given by ID")
                .arg(Arg::new("ID")
                     .help("User ID")
                     .required(true)
                     .index(1)
                     ),
            f: &remove::f
    };
    m.push_subcommand("remove", menu_remove);
    
    let menu_dropall = Subcommand {
            app: App::new("dropall")
                .about("Drop all users from the database")
                .arg(Arg::new("yes")
                     .short('y')
                     .long("yes")
                     .help("Confirmation")
                     .required(true)
                     ),
            f: &dropall::f
    };
    m.push_subcommand("dropall", menu_dropall);
    
    let menu_edit = Subcommand {
            app: App::new("edit")
                .about("Edit user given by ID")
                .arg(Arg::new("ID")
                     .help("User ID")
                     .required(true)
                     .index(1)
                     ),
            f: &edit::f
    };
    m.push_subcommand("edit", menu_edit);
    
    let menu_password = Subcommand {
            app: App::new("password")
                .about("Set/delete/check user password")
                .subcommand(App::new("set")
                     .about("Set new password")
                     .arg(
                         Arg::new("ID")
                         .help("User ID")
                         .required(true)
                         .index(1)
                        )
                 )
                 .subcommand(App::new("check")
                      .about("Check user password")
                      .arg(
                         Arg::new("ID")
                         .help("User ID")
                         .required(true)
                         .index(1)
                        )
                 ),
            f: &password::f
    };
    m.push_subcommand("password", menu_password);
    
    let menu_export_csv = Subcommand {
            app: App::new("export_csv")
                .about("Export users to CSV"),
            f: &export_csv::f
    };
    m.push_subcommand("export_csv", menu_export_csv);
    
    let menu_import = Subcommand {
            app: App::new("import")
                .about("Import users from CSV")
                .arg(Arg::new("FILE")
                     .help("CSV file")
                     .required(true)
                     .index(1)
                     ),
            f: &import::f
    };
    m.push_subcommand("import", menu_import);

    m
}
