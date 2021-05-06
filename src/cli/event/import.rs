use crate::db;
use crate::tools; 

// import events
pub fn f(args: &clap::ArgMatches) {
    let new_events: Vec<db::models::event::NewEvent> = match args.value_of("FILE") {
        Some(f) => tools::import::load_csv(f).unwrap(),
        None => Vec::new(),
    };
    let conn = db::establish_connection();
    for e_new in new_events.iter() {
        let mut insert = true;
        for e_exist in db::models::event::query(&conn) {
            if e_new.datetime == e_exist.datetime && e_new.place == e_exist.place {
                insert = false;
            }
        }
        if insert {
            db::models::event::insert_full(&conn, e_new);
            println!("A new event added to the calendar");
        } else {
            println!("An event with the same place and time already exists!"); 
        }
        
    }
}
