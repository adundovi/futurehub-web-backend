use crate::db;
use crate::tools::cli_edit::{
    edit_line,
};

pub fn f(args: &clap::ArgMatches) {
    match args.subcommand() {
        Some(("set",  args)) => {
            match args.value_of("ID") {
                Some(i) => match i.parse::<i32>() {
                    Ok(i) => set_password(i),
                    Err(_) => print!("ID should be a number"),
                },
                None => print!("No ID given"),
                };
        },
        Some(("check",  args)) => {
            match args.value_of("ID") {
                Some(i) => match i.parse::<i32>() {
                    Ok(i) => check_password(i),
                    Err(_) => print!("ID should be a number"),
                },
                None => print!("No ID given"),
                };
        }
        Some((&_, _)) => print!("No subcommand selected"),
        None => print!("No subcommand selected"),
    }
}   

fn set_password(id: i32) {
    let conn = db::establish_connection();
    let plaintext_password = edit_line(&"".to_string(), "New password");
    db::models::User::update_password(id, plaintext_password, &conn);
}

fn check_password(id: i32) {
    let conn = db::establish_connection();
    
    let user = db::models::User::get(id, &conn).expect("Id not found");
    let plaintext_password = edit_line(&"".to_string(), "Current password");

    let l = db::models::LoginData {
        username_or_email: user.username,
        password: plaintext_password,
    };
    if db::models::User::login(l, &conn).is_some() {
        print!("Password OK!");
    } else {
        print!("Passwords do not match!!");
    }
}
