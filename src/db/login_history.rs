use diesel::{prelude::*, sqlite::SqliteConnection};
use chrono::prelude::*;

use crate::db::models;
use crate::db::sqlite_schema::login_history as login_history;

impl models::LoginHistory {
    pub fn create(username: &str, conn: &SqliteConnection) -> Option<models::LoginHistoryInsertable> {
        if let Ok(user) = models::User::get_user_by_username(username, conn) {
            Some(models::LoginHistoryInsertable {
                user_id: user.id,
                login_timestamp: Utc::now().naive_utc(),
            })
        } else {
            None
        }
    }

    pub fn save_login_history(insert_record: models::LoginHistoryInsertable,
                              conn: &SqliteConnection) -> bool {
        diesel::insert_into(login_history::table)
            .values(&insert_record)
            .execute(conn)
            .is_ok()
    }
}
