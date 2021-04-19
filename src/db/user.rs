use bcrypt::{hash, verify};
use diesel::{prelude::*, sqlite::SqliteConnection};
use chrono::prelude::*;
use uuid::Uuid;

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
    
    pub fn create_with_password(user: models::UserDTO,
                 conn: &SqliteConnection) -> bool {
        
        let hash_password = Some(hash(user.password, DEFAULT_COST).unwrap());

        let user = models::NewUser {
            username: user.username,
            email: user.email,
            password: hash_password,
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
    
    pub fn get_user_by_username(username: &str, conn: &SqliteConnection) -> Result<models::User, diesel::result::Error> {
        users::table
            .filter(users::username.eq(username))
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
            .get_result::<models::User>(conn);

        if user_to_verify.is_err() {
            return None;
        }
        let user_to_verify = user_to_verify.unwrap();

        match user_to_verify.password {
            None => None,
            Some(user_password) => {
                if verify(&login.password, &user_password).unwrap() {
                    if let Some(login_history) = models::LoginHistory::create(&user_to_verify.username, conn) {
                        
                        if !models::LoginHistory::save_login_history(login_history, conn) {
                            return None;
                        }

                        let login_session_str = models::User::generate_login_session();
                        models::User::update_login_session_to_db(&user_to_verify.username, &login_session_str, conn);
                        
                        Some(models::LoginInfo {
                            username: user_to_verify.username,
                            login_session: login_session_str,
                        })
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
        }
    }

    pub fn generate_login_session() -> String {
        Uuid::new_v4().to_simple().to_string()
    }

    pub fn is_valid_login_session(user_token: &UserToken, conn: &SqliteConnection) -> bool {
        users::table
            .filter(users::username.eq(&user_token.user))
            .filter(users::login_session.eq(&user_token.login_session))
            .get_result::<models::User>(conn)
            .is_ok()
    }

    pub fn update_login_session_to_db(username: &str, login_session_str: &str,
                                      conn: &SqliteConnection) -> bool {
        if let Ok(user) = models::User::get_user_by_username(username, conn) {
            diesel::update(users::table.find(user.id))
            .set(users::login_session.eq(login_session_str.to_string()))
            .execute(conn)
            .is_ok()
        } else {
            false
        }
    }
}
