use bcrypt::{hash, verify};
use diesel::{prelude::*, sqlite::SqliteConnection};
use chrono::prelude::*;

use crate::rest::jwt::UserToken;
use crate::db::models;
use crate::db::sqlite_schema::users as users;

const DEFAULT_COST: u32 = 10;

impl models::User {
    pub fn create(username: String,
                 email: String,
                 conn: &SqliteConnection) -> bool {

        let user = models::NewUser {
            username: username,
            email: email,
            password: None,
            login_session: None,
            oib: None,
            name: None,
            surname: None,
            address: None,
            phone: None,
            gender: None,
            birthday: None,
            creation_date: Utc::now().naive_utc(),
        };
        diesel::insert_into(users::table)
            .values(&user)
            .execute(conn)
            .is_ok()
    }
    
    pub fn create_full(user: models::NewUser,
                 conn: &SqliteConnection) -> bool {
        let hashed_pwd = match user.password {
            Some(pwd) => Some(hash(pwd, DEFAULT_COST).unwrap()),
            None => None };
        let user = models::NewUser {
            password: hashed_pwd,
            ..user
        };
        diesel::insert_into(users::table)
            .values(&user)
            .execute(conn)
            .is_ok()
    }

    pub fn query(conn: &SqliteConnection) -> Vec<models::User> {
        users::table
            .load::<models::User>(conn)
            .expect("Error loading user")
    }

    pub fn get(id: i32, conn: &SqliteConnection) -> Result<models::User, diesel::result::Error> {
        users::table
            .filter(users::id.eq(id))
            .first::<models::User>(conn)
    }

    pub fn drop_all(conn: &SqliteConnection) {
        diesel::delete(users::table)
            .execute(conn)
            .expect(&format!("Error removing all users"));
    }

    pub fn remove(id: i32, conn: &SqliteConnection) {
        diesel::delete(users::table.filter(users::id.eq(id)))
            .execute(conn)
            .expect(&format!("Error removing user with id = {}", id));
    }

    pub fn update(user: &models::User, conn: &SqliteConnection) {
        diesel::update(users::table.filter(users::id.eq(user.id)))
        .set((users::username.eq(&user.username),
              users::email.eq(&user.email)
        ))
        .execute(conn)
        .expect(&format!("Error updating user with id = {}", user.id));
    }

    pub fn update_password(id: i32, plaintext_password: String, conn: &SqliteConnection) {
        let hash_password = Some(hash(plaintext_password, DEFAULT_COST).unwrap());
        
        diesel::update(users::table.filter(users::id.eq(id)))
        .set(users::password.eq(&hash_password))
        .execute(conn)
        .expect(&format!("Error updating user password with user id = {}", id));
    }

    pub fn login(login: models::LoginData,
                 conn: &SqliteConnection) -> Option<models::LoginInfo> {
        
        let user_to_verify = users::table
            .filter(users::username.eq(&login.username_or_email))
            .or_filter(users::email.eq(&login.username_or_email))
            .get_result::<models::User>(conn)
            .unwrap();

        match user_to_verify.password {
            None => None,
            Some(user_password) => {
                if verify(&login.password, &user_password).unwrap() {
            /*if let Some(login_history) = LoginHistory::create(&user_to_verify.username, conn) {
                if !LoginHistory::save_login_history(login_history, conn) {
                    return None;
                }
                let login_session_str = User::generate_login_session();
                User::update_login_session_to_db(&user_to_verify.username, &login_session_str, conn);
                Some(LoginInfo {
                    username: user_to_verify.username,
                    login_session: login_session_str,
                })*/
                let login_session_str = "bananko".to_string();
                Some(models::LoginInfo {
                    username: user_to_verify.username,
                    login_session: login_session_str,
                })
                } else {
                    None
                }
            }
        }
    }

    pub fn is_valid_login_session(user_token: &UserToken, conn: &SqliteConnection) -> bool {
        users::table
            .filter(users::username.eq(&user_token.user))
            .filter(users::login_session.eq(&user_token.login_session))
            .get_result::<models::User>(conn)
            .is_ok()
    }
}
    
/*
pub fn get_by_slug(connection: &SqliteConnection, slug: String) -> Result<models::Category, diesel::result::Error> {
    categories::table
        .filter(categories::slug.eq(slug))
        .first::<models::Category>(connection)
}

}*/
