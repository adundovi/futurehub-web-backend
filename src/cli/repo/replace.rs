use crate::db;

pub fn f(args: &clap::ArgMatches) {
        let filepath = match args.value_of("URL") {
            Some(t) => String::from(t),
            None => String::from("no_url"),
        };
        match args.value_of("ID") {
            Some(i) => match i.parse::<i32>() {
                Ok(i) => replace_item(i, filepath),
                Err(_) => print!("ID should be a number"),
            },
            None => print!("No ID given"),
        };
}

fn replace_item(id: i32, filepath: String) {

    let conn = db::establish_connection();
    let item = db::models::repo_items::get(&conn, id).expect("Id not found");
    let mut new_item = item.clone();
    new_item.filepath = filepath;
    
    db::models::repo_items::update(&conn, &new_item);
}
