use crate::db;
use crate::db::model_traits::Queries;

pub fn f(_args: &clap::ArgMatches) {
    let conn = db::establish_connection();
    for p in db::models::user::User::get_all(&conn).expect("Not found") {
        println!("{}\t{}\t{}\t{}\t{}",
                 p.id, p.username, p.email,
                 p.oib.unwrap_or_default(),
                 p.address.unwrap_or_default());
    }
}
