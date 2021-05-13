use crate::db;

pub fn f(args: &clap::ArgMatches) {
    let conn = db::establish_connection();

    match args.subcommand() {
        Some(("add",  args)) => {
            let eid = match args.value_of("EID") {
                Some(t) => t.parse::<i32>().ok(),
                None => None,
            };
            let uid = match args.value_of("UID") {
                Some(t) => t.parse::<i32>().ok(),
                None => None,
            };

            if eid.is_some() && uid.is_some() {
                db::models::event::add_attendee(eid.unwrap(), uid.unwrap(), &conn);
            }
       },
       Some(("list",  args)) => {
            match args.value_of("ID") {
                Some(i) => match i.parse::<i32>() {
                    Ok(i) => list_attendees(i),
                    Err(_) => print!("ID should be a number"),
                },
                None => print!("No ID given"),
            };
       },
       Some(("remove",  args)) => {
            let eid = match args.value_of("EID") {
                Some(t) => t.parse::<i32>().ok(),
                None => None,
            };
            let uid = match args.value_of("UID") {
                Some(t) => t.parse::<i32>().ok(),
                None => None,
            };

            if eid.is_some() && uid.is_some() {
                db::models::event::remove_attendee(eid.unwrap(), uid.unwrap(), &conn);
            }
       },
        Some((&_, _)) => print!("No subcommand selected"),
        None => print!("No subcommand selected"),
    }
}

fn list_attendees(id: i32) {
    let conn = db::establish_connection();
    for (i, p) in db::models::event::list_attendees(id, &conn).iter().enumerate() {
                let presence = match p.1.presence.as_ref() {
                    Some(p) => p,
                    None => "",
                };
                println!("{}.\t{}({})\t{}", i+1, p.0.username, p.0.id, presence);
    }
}
