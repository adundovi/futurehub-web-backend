use crate::db;

pub fn f(_args: &clap::ArgMatches) {
    let conn = db::establish_connection();
    for p in db::models::course::Course::query(&conn) {
        println!("{}\t{}\t{}", p.id, p.code, p.title);
    }
}
