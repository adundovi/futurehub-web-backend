use clap::{Arg, App};
use std::collections::HashMap;

use crate::cli::menu::{Menu, Subcommand};

mod make_folders;

pub fn menu<'a>() -> Menu<'a> {
    let mut m = Menu{
        name: "zns",
        about: "ZNS helpers",
        author: "Andrej Dundović <andrej.dundovic@udruga-point.hr>",
        version: "0.1",
        subcommands: HashMap::new()
    };

    let menu_make_folders = Subcommand {
            app: App::new("make_folders")
                .about("Make the folder structure")
                .arg(Arg::new("PATH")
                     .about("PATH to the directory")
                     .required(true)
                     .index(1)
                     ),
            f: &make_folders::f
    };
    m.push_subcommand("make_folders", menu_make_folders);
    
    m
}
