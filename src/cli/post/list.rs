use crate::db;

// list posts
pub fn f(_args: &clap::ArgMatches) {
    let conn = db::establish_connection();
    for p in db::models::post::get_all(&conn) {
        println!("{}\t{}\t{}", p.id, p.title, p.datetime);
    }
}
