use super::sqlite_schema::events;
use chrono::NaiveDateTime;
use diesel::sql_types::Timestamp;

use super::super::tools::import;

#[derive(Debug, Insertable, Serialize, Deserialize, Eq, Ord, PartialEq, PartialOrd)]
#[table_name = "events"]
#[serde(rename_all = "PascalCase")]
pub struct NewEvent {
    #[serde(with= "import::date_serializer")]
    pub datetime: NaiveDateTime,
    pub title: String,
    pub body: Option<String>,
    pub place: Option<String>,
    pub audience: Option<String>,
}

#[derive(Queryable)]
pub struct Event {
    pub id: i32,
    pub title: String,
    pub body: Option<String>,
    pub place: Option<String>,
    pub audience: Option<String>,
    pub datetime: NaiveDateTime, // UTC
}


