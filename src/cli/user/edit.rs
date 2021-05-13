use crate::db;
use crate::db::model_traits::Queries;
use crate::tools::cli_edit::{
    edit_line,
    edit_option_line,
    edit_option_datetime,
};

pub fn f(args: &clap::ArgMatches) {
        match args.value_of("ID") {
            Some(i) => match i.parse::<i32>() {
                Ok(i) => edit_item(i),
                Err(_) => print!("ID should be a number"),
            },
            None => print!("No ID given"),
            };
    }

fn edit_item(id: i32) {

    let conn = db::establish_connection();
    
    let item = db::models::user::User::get(&conn, id).expect("Id not found");
    let mut new_item = item.clone();

    new_item.username = edit_line(&item.username, "Username");
    new_item.email = edit_line(&item.email, "Email");
    new_item.oib = edit_option_line(&item.oib, "OIB");
    new_item.name = edit_option_line(&item.name, "Name");
    new_item.surname = edit_option_line(&item.surname, "Surname");
    new_item.address = edit_option_line(&item.address, "Address");
    new_item.phone = edit_option_line(&item.phone, "Phone");
    new_item.birthday = edit_option_datetime(&item.birthday, "Birthday");
    new_item.gender = edit_option_line(&item.gender, "Gender");
    
    db::models::user::User::update(&new_item, &conn);
}
