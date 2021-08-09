use crate::db;

pub fn f(args: &clap::ArgMatches) {
    let conn = db::establish_connection();

    match args.subcommand() {
        Some(("add",  args)) => {
            let cid = match args.value_of("CID") {
                Some(t) => t.parse::<i32>().ok(),
                None => None,
            };
            let uid = match args.value_of("UID") {
                Some(t) => t.parse::<i32>().ok(),
                None => None,
            };

            if cid.is_some() && uid.is_some() {
                db::models::course::Course::add_participant(cid.unwrap(), uid.unwrap(), &conn);
            }
       },
       Some(("list",  args)) => {
            match args.value_of("ID") {
                Some(i) => match i.parse::<i32>() {
                    Ok(i) => list_participants(i),
                    Err(_) => print!("ID should be a number"),
                },
                None => print!("No ID given"),
            };
       },
       Some(("remove",  args)) => {
            let cid = match args.value_of("CID") {
                Some(t) => t.parse::<i32>().ok(),
                None => None,
            };
            let uid = match args.value_of("UID") {
                Some(t) => t.parse::<i32>().ok(),
                None => None,
            };

            if cid.is_some() && uid.is_some() {
                db::models::course::Course::remove_participant(cid.unwrap(), uid.unwrap(), &conn);
            }
       },
        Some((&_, _)) => print!("No subcommand selected"),
        None => print!("No subcommand selected"),
    }
}

fn list_participants(id: i32) {
    let conn = db::establish_connection();
    for (i, p) in db::models::course::Course::list_participants(&conn, id).iter().enumerate() {
                println!("{}.\t{}({})\t{}", i+1, p.0.username, p.0.id, p.1.join_date);
    }
}
