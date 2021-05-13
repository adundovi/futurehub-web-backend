use crate::db;
use crate::db::model_traits::Queries;

pub fn f(_args: &clap::ArgMatches) {
    let conn = db::establish_connection();
    for p in db::models::course::Course::get_all(&conn).expect("Not found") {
        println!("{}\t{}\t{}", p.id, p.code, p.title);
    }
}
