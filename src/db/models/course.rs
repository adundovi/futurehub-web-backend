use crate::db::sqlite_schema::courses as courses;
use crate::tools::import;

use chrono::NaiveDateTime;
use diesel::{prelude::*, sqlite::SqliteConnection};
use chrono::prelude::*;

#[derive(Debug, Insertable, Serialize, Deserialize, Eq, Ord, PartialEq, PartialOrd)]
#[table_name = "courses"]
#[serde(rename_all = "PascalCase")]
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

impl Course {
    pub fn create(title_: String,
               code_: String,
               conn: &SqliteConnection) -> bool {

        let course = NewCourse {
            code: code_,
            title: title_,
            description: None,
            creation_date: Utc::now().naive_utc(),
            cert_template: None,
            lecturer: None,
            organizer: None,
            lectures: None,
            students: None,
            max_students: None,
            finished: false,
            published: false
        };
        diesel::insert_into(courses::table)
            .values(&course)
            .execute(conn)
            .is_ok()
    }
    
    pub fn query(conn: &SqliteConnection) -> Vec<Course> {
        courses::table
            .load::<Course>(conn)
            .expect("Error loading course")
    }

    pub fn get(id: i32, conn: &SqliteConnection) -> Result<Course, diesel::result::Error> {
        courses::table
            .filter(courses::id.eq(id))
            .first::<Course>(conn)
    }
    
    pub fn remove(id: i32, conn: &SqliteConnection) {
        diesel::delete(courses::table.filter(courses::id.eq(id)))
            .execute(conn)
            .expect(&format!("Error removing course with id = {}", id));
    }
    
    pub fn drop_all(conn: &SqliteConnection) {
        diesel::delete(courses::table)
            .execute(conn)
            .expect(&format!("Error removing all courses"));
    }
    
    pub fn update(course: &Course, conn: &SqliteConnection) {
        diesel::update(courses::table.filter(courses::id.eq(course.id)))
        .set((courses::code.eq(&course.code),
              courses::title.eq(&course.title),
              courses::description.eq(&course.description),
              courses::creation_date.eq(&course.creation_date),
              courses::cert_template.eq(&course.cert_template),
              courses::lecturer.eq(&course.lecturer),
              courses::organizer.eq(&course.organizer),
              courses::lectures.eq(&course.lectures),
              courses::students.eq(&course.students),
              courses::max_students.eq(&course.max_students),
              courses::finished.eq(&course.finished),
              courses::published.eq(&course.published),
        ))
        .execute(conn)
        .expect(&format!("Error updating course with id = {}", course.id));
    }

}
