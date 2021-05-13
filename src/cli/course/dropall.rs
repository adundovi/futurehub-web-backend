use crate::db;
use crate::db::model_traits::Queries;

pub fn f(args: &clap::ArgMatches) {
    if args.is_present("yes") {
            let conn = db::establish_connection();
            db::models::course::Course::drop_all(&conn).expect("Error");
        } else {
            print!("Please confirm the action with --yes");
    }
}
