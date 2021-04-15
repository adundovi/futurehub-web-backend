use crate::db;

pub fn f(_args: &clap::ArgMatches) {
    let conn = db::establish_connection();
    for p in db::models::User::query(&conn) {
        println!("{}\t{}\t{}", p.id, p.username, p.email);
    }
}
