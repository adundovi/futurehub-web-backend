use clap::{Arg, App};

pub mod event;

pub fn menu_main<'a>() -> App<'a> {
    App::new("CLI for FHK webapp")
        .version("0.1")
        .author("Andrej Dundović <andrej.dundovic@udruga-point.hr>")
        .about("To interact with webapp through command line interface")
        .arg(Arg::new("config")
            .short('c')
            .long("config")
            .value_name("FILE")
            .about("Sets a custom config file")
            .takes_value(true))
        .arg(Arg::new("v")
            .short('v')
            .multiple(true)
            .about("Sets the level of verbosity"))

}

