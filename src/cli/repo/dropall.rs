use crate::db;

// dropall repository items
pub fn f(args: &clap::ArgMatches) {
    if args.is_present("yes") {
            let conn = db::establish_connection();
            db::models::repo_items::drop_all(&conn);
        } else {
            print!("Please confirm the action with --yes");
    }
}
