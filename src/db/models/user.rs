use bcrypt::{hash, verify};
use diesel::{prelude::*, sqlite::SqliteConnection};
use chrono::prelude::*;
use uuid::Uuid;

use crate::rest::jwt::UserToken;
use crate::db::model_traits::Queries;
use crate::db::models::login_history::LoginHistory;
use crate::db::sqlite_schema::users as users;
use crate::tools::import;

const DEFAULT_COST: u32 = 10;

#[derive(Serialize, Deserialize)]
pub struct LoginData {
    pub username_or_email: String,
    pub password: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct LoginInfo {
    pub username: String,
    pub login_session: String,
}

#[derive(Queryable, Serialize, Clone)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: Option<String>,
    pub login_session: Option<String>,
    pub oib: Option<String>,
    pub name: Option<String>,
    pub surname: Option<String>,
    pub address: Option<String>,
    pub phone: Option<String>,
    pub gender: Option<String>,
    pub birthday: Option<NaiveDateTime>,
    pub creation_date: NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "users"]
pub struct UserDTO {
    pub username: String,
    pub email: String,
    pub password: String,
}


#[derive(Debug, Insertable, Serialize, Deserialize, Eq, Ord, PartialEq, PartialOrd)]
#[table_name = "users"]
#[serde(rename_all = "PascalCase")]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password: Option<String>,
    pub login_session: Option<String>,
    pub oib: Option<String>,
    pub name: Option<String>,
    pub surname: Option<String>,
    pub address: Option<String>,
    pub phone: Option<String>,
    pub gender: Option<String>,
    pub birthday: Option<NaiveDateTime>,
    #[serde(with= "import::date_serializer")]
    pub creation_date: NaiveDateTime,
}

impl Queries for User {
    fn get_all(conn: &SqliteConnection) -> Result<Vec<User>, diesel::result::Error> {
        users::table
            .load::<User>(conn)
    }
    
    fn get(conn: &SqliteConnection, id: i32) -> Result<User, diesel::result::Error> {
        users::table
            .filter(users::id.eq(id))
            .first::<User>(conn)
    }
    
    fn drop_all(conn: &SqliteConnection) -> Result<usize, diesel::result::Error>{
        diesel::delete(users::table)
            .execute(conn)
    }

    fn remove(conn: &SqliteConnection, id: i32) -> Result<usize, diesel::result::Error>{
        diesel::delete(users::table.filter(users::id.eq(id)))
            .execute(conn)
    }

}

impl User {
    pub fn create(username: String,
                 email: String,
                 conn: &SqliteConnection) -> bool {

        let user = NewUser {
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
    
    pub fn create_with_password(user: UserDTO,
                 conn: &SqliteConnection) -> bool {
        
        let hash_password = Some(hash(user.password, DEFAULT_COST).unwrap());

        let user = NewUser {
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
    
    pub fn create_full(user: NewUser,
                 conn: &SqliteConnection) -> bool {
        let hashed_pwd = match user.password {
            Some(pwd) => Some(hash(pwd, DEFAULT_COST).unwrap()),
            None => None };
        let user = NewUser {
            password: hashed_pwd,
            ..user
        };
        diesel::insert_into(users::table)
            .values(&user)
            .execute(conn)
            .is_ok()
    }

    pub fn get_user_by_username(username: &str, conn: &SqliteConnection) -> Result<User, diesel::result::Error> {
        users::table
            .filter(users::username.eq(username))
            .first::<User>(conn)
    }

    pub fn update(user: &User, conn: &SqliteConnection) {
        diesel::update(users::table.filter(users::id.eq(user.id)))
        .set((users::username.eq(&user.username),
              users::email.eq(&user.email),
              users::oib.eq(&user.oib),
              users::name.eq(&user.name),
              users::surname.eq(&user.surname),
              users::address.eq(&user.address),
              users::phone.eq(&user.phone),
              users::gender.eq(&user.gender),
              users::birthday.eq(&user.birthday),
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

    pub fn login(login: LoginData,
                 conn: &SqliteConnection) -> Option<LoginInfo> {
        
        let user_to_verify = users::table
            .filter(users::username.eq(&login.username_or_email))
            .or_filter(users::email.eq(&login.username_or_email))
            .get_result::<User>(conn);

        if user_to_verify.is_err() {
            return None;
        }
        let user_to_verify = user_to_verify.unwrap();

        match user_to_verify.password {
            None => None,
            Some(user_password) => {
                if verify(&login.password, &user_password).unwrap() {
                    if let Some(login_history) = LoginHistory::create(&user_to_verify.username, conn) {
                        
                        if !LoginHistory::save_login_history(login_history, conn) {
                            return None;
                        }

                        let login_session_str = User::generate_login_session();
                        User::update_login_session_to_db(&user_to_verify.username, &login_session_str, conn);
                        
                        Some(LoginInfo {
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
            .get_result::<User>(conn)
            .is_ok()
    }

    pub fn update_login_session_to_db(username: &str, login_session_str: &str,
                                      conn: &SqliteConnection) -> bool {
        if let Ok(user) = User::get_user_by_username(username, conn) {
            diesel::update(users::table.find(user.id))
            .set(users::login_session.eq(login_session_str.to_string()))
            .execute(conn)
            .is_ok()
        } else {
            false
        }
    }
}
