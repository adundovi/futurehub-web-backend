use crate::db;

// create post
pub fn f(args: &clap::ArgMatches) {
    let filepath = match args.value_of("URL") {
        Some(t) => String::from(t),
        None => String::from("no_url"),
    };
}
