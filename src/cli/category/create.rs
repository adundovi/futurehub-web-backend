use crate::db;

// create post
pub fn f(args: &clap::ArgMatches) {
    let title = match args.value_of("TITLE") {
        Some(t) => String::from(t),
        None => String::from("Empty title"),
    };
    let conn = db::establish_connection();
    db::models::category::insert(&conn, title);
}
