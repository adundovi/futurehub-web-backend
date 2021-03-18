use crate::db;

// list categories
pub fn f(_args: &clap::ArgMatches) {
    let conn = db::establish_connection();
    for p in db::category::query(&conn) {
        println!("{}\t{}\t{}", p.id, p.title, p.slug);
    }
}
