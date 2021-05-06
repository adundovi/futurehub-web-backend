use crate::db;

pub fn f(args: &clap::ArgMatches) {
    
    let course = match args.value_of("TITLE") {
        Some(t) => String::from(t),
        None => String::from("No_title"),
    };
    let code = match args.value_of("CODE") {
        Some(t) => String::from(t),
        None => String::from("no_code"),
    };

    let conn = db::establish_connection();
    db::models::course::Course::create(course, code, &conn);
}
