use diesel::{prelude::*, sqlite::SqliteConnection};
use chrono::prelude::*;

use super::user::User;
use crate::db::sqlite_schema::login_history as login_history;
use crate::tools::import;

#[derive(Debug, Insertable, Serialize, Deserialize, Eq, Ord, PartialEq, PartialOrd)]
//#[belongs_to(User)]
#[table_name = "login_history"]
pub struct LoginHistory {
    pub id: i32,
    pub user_id: i32,
    #[serde(with= "import::date_serializer")]
    pub login_timestamp: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "login_history"]
pub struct LoginHistoryInsertable {
    pub user_id: i32,
    pub login_timestamp: NaiveDateTime,
}

impl LoginHistory {
    pub fn create(username: &str, conn: &SqliteConnection) -> Option<LoginHistoryInsertable> {
        if let Ok(user) = User::get_user_by_username(username, conn) {
            Some(LoginHistoryInsertable {
                user_id: user.id,
                login_timestamp: Utc::now().naive_utc(),
            })
        } else {
            None
        }
    }

    pub fn save_login_history(insert_record: LoginHistoryInsertable,
                              conn: &SqliteConnection) -> bool {
        diesel::insert_into(login_history::table)
            .values(&insert_record)
            .execute(conn)
            .is_ok()
    }
}
