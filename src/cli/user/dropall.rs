use crate::db;

pub fn f(args: &clap::ArgMatches) {
    if args.is_present("yes") {
            let conn = db::establish_connection();
            db::models::User::drop_all(&conn);
        } else {
            print!("Please confirm the action with --yes");
    }
}
