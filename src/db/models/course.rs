use crate::db::sqlite_schema::courses as courses;
use crate::tools::import;

use chrono::NaiveDateTime;

#[derive(Queryable, Serialize, Clone)]
pub struct NewCourse {
    pub code: String,
    pub title: String,
    pub description: Option<String>,
    #[serde(with= "import::date_serializer")]
    pub creation_date: NaiveDateTime,
    pub cert_template: Option<String>,
    pub lecturer: Option<String>,
    pub organizer: Option<String>,
    pub lectures: Option<i32>,
    pub students: Option<i32>,
    pub max_students: Option<i32>,
    pub finished: bool,
    pub published: bool,
}

#[derive(Queryable, Serialize, Clone)]
pub struct Course {
    pub id: i32,
    pub code: String,
    pub title: String,
    pub description: Option<String>,
    #[serde(with= "import::date_serializer")]
    pub creation_date: NaiveDateTime,
    pub cert_template: Option<String>,
    pub lecturer: Option<String>,
    pub organizer: Option<String>,
    pub lectures: Option<i32>,
    pub students: Option<i32>,
    pub max_students: Option<i32>,
    pub finished: bool,
    pub published: bool,
}

