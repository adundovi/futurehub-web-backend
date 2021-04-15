use super::sqlite_schema::{
    events,
    posts,
    repo_items,
    categories,
    users,
    login_history,
};
use chrono::NaiveDateTime;

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

#[derive(Queryable, Serialize, Deserialize, Clone)]
pub struct Event {
    pub id: i32,
    pub title: String,
    pub body: Option<String>,
    pub place: Option<String>,
    pub audience: Option<String>,
    pub datetime: NaiveDateTime, // UTC
}

#[derive(Debug, Insertable, Serialize, Deserialize, Eq, Ord, PartialEq, PartialOrd)]
#[table_name = "posts"]
#[serde(rename_all = "PascalCase")]
pub struct NewPost {
    #[serde(with= "import::date_serializer")]
    pub datetime: NaiveDateTime,
    pub title: String,
    pub slug: String,
    pub body: Option<String>,
    pub published: bool,
}

#[derive(Queryable, Clone)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub body: Option<String>,
    pub published: bool,
    pub datetime: NaiveDateTime, // UTC
}

#[derive(Debug, Insertable, Serialize, Deserialize, Eq, Ord, PartialEq, PartialOrd)]
#[table_name = "repo_items"]
#[serde(rename_all = "PascalCase")]
pub struct NewRepoItem {
    #[serde(with= "import::date_serializer")]
    pub datetime: NaiveDateTime,
    pub title: String,
    pub slug: String,
    pub filepath: String,
    pub description: Option<String>,
    pub category_id: i32,
    pub filetype: Option<String>,
    pub published: bool
}

#[derive(Queryable, Clone)]
pub struct RepoItem {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub filepath: String,
    pub description: Option<String>,
    pub category_id: i32,
    pub filetype: Option<String>,
    pub published: bool,
    pub datetime: NaiveDateTime, // UTC
}

#[derive(Debug, Insertable, Serialize, Deserialize, Eq, Ord, PartialEq, PartialOrd)]
#[table_name = "categories"]
#[serde(rename_all = "PascalCase")]
pub struct NewCategory {
    pub title: String,
    pub slug: String,
    pub icon: Option<String>,
    pub description: Option<String>,
}

#[derive(Queryable, Clone)]
pub struct Category {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub icon: Option<String>,
    pub description: Option<String>,
}

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
    pub creation_date: NaiveDateTime,
}

#[derive(Queryable, Clone)]
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
