use crate::db;

pub fn f(args: &clap::ArgMatches) {
    let conn = db::establish_connection();

    match args.subcommand() {
        Some(("add",  args)) => {
            let cid = match args.value_of("CID") {
                Some(t) => t.parse::<i32>().ok(),
                None => None,
            };
            let eid = match args.value_of("EID") {
                Some(t) => t.parse::<i32>().ok(),
                None => None,
            };

            if cid.is_some() && eid.is_some() {
                db::models::course::Course::add_event(cid.unwrap(), eid.unwrap(), &conn);
            }
       },
       Some(("list",  args)) => {
            match args.value_of("CID") {
                Some(i) => match i.parse::<i32>() {
                    Ok(i) => list_events(i),
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
            let eid = match args.value_of("EID") {
                Some(t) => t.parse::<i32>().ok(),
                None => None,
            };

            if cid.is_some() && eid.is_some() {
                db::models::course::Course::remove_event(cid.unwrap(), eid.unwrap(), &conn);
            }
       },
        Some((&_, _)) => print!("No subcommand selected"),
        None => print!("No subcommand selected"),
    }
}

fn list_events(id: i32) {
    let conn = db::establish_connection();
    for (i, p) in db::models::course::Course::list_events(&conn, id).iter().enumerate() {
                println!("{}.\t{}({})\t{}", i+1, p.0.title, p.0.id, p.0.datetime);
    }
}
