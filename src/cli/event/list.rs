use crate::db;

// list events
pub fn f(_args: &clap::ArgMatches) {
    let conn = db::establish_connection();
    for p in db::event::query(&conn) {
        println!("{}\t{}\t{}", p.id, p.title, p.datetime);
    }
}
