use crate::db;
use std::path::Path;
use chrono::prelude::*;

// create repo item
pub fn f(args: &clap::ArgMatches) {
    let cat: i32 = match args.value_of("category") {
        Some(t) => t.parse::<i32>().unwrap(),
        None => 0,
    };
    let publish: bool = args.is_present("publish");
    
    let conn = db::establish_connection();
    
    match args.values_of("URL") {
        Some(values) => {
            let utc_now = Utc::now();
            for v in values {
                let title = match args.value_of("title") {
                    Some(t) => String::from(t),
                    None => String::from(
                        Path::new(&v).file_stem().unwrap().to_str().unwrap()
                    ),
                };
                db::models::repo_items::insert(&conn, title, v.to_string(), &utc_now, cat, publish);
            }
        },
        None => println!("Error: no url is given")
    };
}
