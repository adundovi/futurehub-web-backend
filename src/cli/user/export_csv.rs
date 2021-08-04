use crate::db;
use crate::tools::export::save_csv;
use crate::db::model_traits::Queries;
use std::error::Error;

pub fn f(_args: &clap::ArgMatches) {
    let users = load_users();
    match users {
        Ok(users) => save_csv(users).unwrap(),
        Err(_) => println!("Something went wrong :-p"),
    }
}

fn load_users() -> Result<Vec<db::models::user::User>, Box<dyn Error>> {
    let conn = db::establish_connection();
    let users = db::models::user::User::get_all(&conn).expect("Not found");
    Ok(users)
}
