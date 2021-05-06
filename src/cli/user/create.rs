use crate::db;

pub fn f(args: &clap::ArgMatches) {
    let username = match args.value_of("USERNAME") {
        Some(t) => String::from(t),
        None => String::from("No_username"),
    };
    let email = match args.value_of("EMAIL") {
        Some(t) => String::from(t),
        None => String::from("no_email"),
    };
    let conn = db::establish_connection();
    db::models::user::User::create(username, email, &conn);
}
