use crate::db;

// list repository items
pub fn f(_args: &clap::ArgMatches) {
    let conn = db::establish_connection();
    for p in db::models::repo_items::query(&conn) {
        println!("{}\t{}\t{}\t{}\t{}", p.id, p.title, p.filepath, p.datetime, p.slug);
    }
}
