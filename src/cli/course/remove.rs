use crate::db;
use crate::db::model_traits::Queries;

// rm repo item
pub fn f(args: &clap::ArgMatches) {
    match args.value_of("ID") {
        Some(id) => match id.parse::<i32>() {
            Ok(id) => {
                let conn = db::establish_connection();
                db::models::course::Course::remove(&conn, id).expect("Error");
            },
            Err(_) => print!("ID should be a number"),
        },
        None => print!("No ID given"),
    };
}
