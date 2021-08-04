use crate::db;
use crate::tools; 

// import events
pub fn f(args: &clap::ArgMatches) {
    let imported_users: Vec<db::models::user::NewUser> = match args.value_of("FILE") {
        Some(f) => tools::import::load_csv(f).unwrap(),
        None => Vec::new(),
    };
    let conn = db::establish_connection();
    
    for u in imported_users.into_iter() {
        if db::models::user::User::get_user_by_username(&conn, &u.username).is_ok() {
            continue;
        }
        println!("User added: {}", &u.username);
        db::models::user::User::create_full(&conn, u);
    }
}
