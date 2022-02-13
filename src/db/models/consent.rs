use diesel::{prelude::*, sqlite::SqliteConnection};
use chrono::prelude::*;

use crate::tools::import;
use crate::db::sqlite_schema::consents as consents;
use crate::tools;

#[derive(Debug, Insertable, Serialize, Deserialize, Eq, Ord, PartialEq, PartialOrd)]
#[table_name = "consents"]
#[serde(rename_all = "PascalCase")]
pub struct NewConsent {
    pub name: String,
    pub surname: String,
    pub email: String,
    pub phone: Option<String>,
    pub oib: String,
    pub child_name: String,
    pub child_surname: String,
    pub consent_on_off: String,
    pub consent_type: String,
    #[serde(with= "import::date_serializer")]
    pub entry_date: NaiveDateTime,
    pub verified: bool,
    pub verify_hash: Option<String>
}

#[derive(Queryable, Clone)]
pub struct Consent {
    pub id: i32,
    pub name: String,
    pub surname: String,
    pub email: String,
    pub phone: Option<String>,
    pub oib: String,
    pub child_name: String,
    pub child_surname: String,
    pub consent_on_off: String,
    pub consent_type: String,
    pub entry_date: NaiveDateTime,
    pub verified: bool,
    pub verify_hash: Option<String>
}

impl Consent {
    pub fn insert(conn: &SqliteConnection,
                  consent: &NewConsent) -> bool {
        diesel::insert_into(consents::table)
            .values(consent)
            .execute(conn)
            .is_ok()
    }
}
