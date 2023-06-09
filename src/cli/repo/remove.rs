use crate::db;

// rm repo item
pub fn f(args: &clap::ArgMatches) {
    match args.value_of("ID") {
        Some(id) => match id.parse::<i32>() {
            Ok(id) => {
                let conn = db::establish_connection();
                db::models::repo_items::remove(&conn, id);
            },
            Err(_) => print!("ID should be a number"),
        },
        None => print!("No ID given"),
    };
}
