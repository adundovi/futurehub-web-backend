use super::sqlite_schema::{events, posts, repo_items};
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

#[derive(Queryable, Clone)]
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
    pub category: Option<String>,
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
    pub category: Option<String>,
    pub filetype: Option<String>,
    pub published: bool,
    pub datetime: NaiveDateTime, // UTC
}
