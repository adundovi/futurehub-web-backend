use crate::db;

pub fn f(_args: &clap::ArgMatches) {
    let conn = db::establish_connection();
    for p in db::models::user::User::query(&conn) {
        println!("{}\t{}\t{}\t{}\t{}",
                 p.id, p.username, p.email,
                 p.oib.unwrap_or_default(),
                 p.address.unwrap_or_default());
    }
}
