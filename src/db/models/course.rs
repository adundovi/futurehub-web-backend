use crate::db::sqlite_schema::courses as courses;
use crate::db::sqlite_schema::course_users as cusers;
use crate::db::sqlite_schema::users as users;
use crate::db::sqlite_schema::events as events;
use crate::db::model_traits::Queries;
use crate::db::models::user::User;
use crate::db::models::event::Event;
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
    pub lecture_duration: Option<i32>,
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
    pub lecture_duration: Option<i32>,
    pub students: Option<i32>,
    pub max_students: Option<i32>,
    pub finished: bool,
    pub published: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CourseAttribs {
    pub title: String,
    pub code: String,
    pub description: Option<String>,
    pub creation_date: NaiveDateTime,
    pub lecturer: Option<String>,
    pub organizer: Option<String>,
    pub lectures: Option<i32>,
    pub lecture_duration: Option<i32>,
    pub students: Option<i32>,
    pub max_students: Option<i32>,
    pub finished: bool,
    pub published: bool,
}

#[derive(Debug, Insertable, Serialize, Deserialize, Eq, Ord, PartialEq, PartialOrd)]
#[table_name = "cusers"]
#[serde(rename_all = "PascalCase")]
pub struct NewCourseUser {
    pub course_id: i32,
    pub user_id: i32,
    #[serde(with= "import::date_serializer")]
    pub join_date: NaiveDateTime,
    pub leave_date: Option<NaiveDateTime>,
    pub score: Option<i32>,
    pub attendance: Option<i32>,
    pub note: Option<String>,
}

#[derive(Queryable, Serialize, Clone)]
pub struct CourseUser {
    pub id: i32,
    pub course_id: i32,
    pub user_id: i32,
    #[serde(with= "import::date_serializer")]
    pub join_date: NaiveDateTime,
    pub leave_date: Option<NaiveDateTime>,
    pub score: Option<i32>,
    pub attendance: Option<i32>,
    pub note: Option<String>,
}

impl Queries for Course {
    fn get_all(conn: &SqliteConnection) -> QueryResult<Vec<Course>> {
        courses::table
            .load::<Course>(conn)
    }
    
    fn get(conn: &SqliteConnection, id: i32) -> QueryResult<Course> {
        courses::table
            .filter(courses::id.eq(id))
            .first::<Course>(conn)
    }
    
    fn remove(conn: &SqliteConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(courses::table.filter(courses::id.eq(id)))
            .execute(conn)
    }
    
    fn drop_all(conn: &SqliteConnection) -> QueryResult<usize> {
        diesel::delete(courses::table)
            .execute(conn)
    }
    
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
            lecture_duration: None,
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
    
    pub fn create_full(conn: &SqliteConnection,
                       course: NewCourse) -> bool {
        diesel::insert_into(courses::table)
            .values(course)
            .execute(conn)
            .is_ok()
    }
    
    pub fn get_published(conn: &SqliteConnection, id: i32) -> QueryResult<Course> {
        courses::table
            .filter(courses::id.eq(id).and(courses::published.eq(true)))
            .first::<Course>(conn)
    }
    
    pub fn get_by_code(conn: &SqliteConnection, code: &str) -> QueryResult<Course> {
        courses::table
            .filter(courses::code.eq(code))
            .first::<Course>(conn)
    }
    
    pub fn get_published_by_code(conn: &SqliteConnection, code: &str) -> QueryResult<Course> {
        courses::table
            .filter(courses::code.eq(code).and(courses::published.eq(true)))
            .first::<Course>(conn)
    }

    pub fn get_all_published(conn: &SqliteConnection) -> QueryResult<Vec<Course>> {
        courses::table
            .filter(courses::published.eq(true))
            .order(courses::creation_date.asc())
            .load::<Course>(conn)
    }
    
    pub fn update(conn: &SqliteConnection, course: &Course) {
        diesel::update(courses::table.filter(courses::id.eq(course.id)))
        .set((courses::code.eq(&course.code),
              courses::title.eq(&course.title),
              courses::description.eq(&course.description),
              courses::creation_date.eq(&course.creation_date),
              courses::cert_template.eq(&course.cert_template),
              courses::lecturer.eq(&course.lecturer),
              courses::organizer.eq(&course.organizer),
              courses::lectures.eq(&course.lectures),
              courses::lecture_duration.eq(&course.lecture_duration),
              courses::students.eq(&course.students),
              courses::max_students.eq(&course.max_students),
              courses::finished.eq(&course.finished),
              courses::published.eq(&course.published),
        ))
        .execute(conn)
        .expect(&format!("Error updating course with id = {}", course.id));
    }
    
    pub fn set_students(conn: &SqliteConnection, id: i32, students: i32) {
        diesel::update(courses::table.filter(courses::id.eq(id)))
        .set((courses::students.eq(students),))
        .execute(conn)
        .expect(&format!("Error updating students in course with id = {}", id));
    }

    pub fn add_participant(course_id_: i32,
                           user_id_: i32,
                           conn: &SqliteConnection) -> bool {
        
        let relation = NewCourseUser {
            course_id: course_id_,
            user_id: user_id_,
            join_date: Utc::now().naive_utc(),
            leave_date: None,
            score: None,
            attendance: None,
            note: None,
        };
        diesel::insert_into(cusers::table)
            .values(&relation)
            .execute(conn)
            .is_ok()
    }
   
    pub fn list_participants(conn: &SqliteConnection, id: i32) -> Vec<(User, CourseUser)> {
        users::table
            .inner_join(cusers::table)
            .filter(cusers::course_id.eq(id))
            .load::<(User, CourseUser)>(conn)
            .expect("Error loading course users")
    }

    pub fn remove_participant(course_id_: i32,
                              user_id_: i32,
                              conn: &SqliteConnection) {
        
        diesel::delete(cusers::table.filter(cusers::course_id.eq(course_id_)
                                            .and(cusers::user_id.eq(user_id_))))
            .execute(conn)
            .expect(&format!("Error relation user-course course_id = {}, user_id = {}",
                             course_id_, user_id_));
    }
    
    pub fn add_event(course_id_: i32,
                     event_id_: i32,
                     conn: &SqliteConnection) -> bool {
        diesel::update(events::table)
            .filter(events::id.eq(event_id_))
            .set(events::course_id.eq(course_id_))
            .execute(conn)
            .is_ok()
    }
    
    pub fn remove_event(course_id_: i32,
                        event_id_: i32,
                        conn: &SqliteConnection) {
        
        diesel::delete(events::table.filter(events::course_id.eq(course_id_)
                                            .and(events::id.eq(event_id_))))
            .execute(conn)
            .expect(&format!("Error relation event-course course_id = {}, event_id = {}",
                             course_id_, event_id_));
    }
    
    pub fn list_events(conn: &SqliteConnection, id: i32) -> Vec<Event> {
        events::table
            .filter(events::course_id.eq(id))
            .load::<Event>(conn)
            .expect("Error loading course users")
    }
    
    pub fn first_date(id: i32, conn: &SqliteConnection) -> NaiveDateTime {
        let e = events::table
            .filter(events::course_id.eq(id))
            .order(events::datetime.asc())
            .first::<Event>(conn)
            .expect("Error loading course users");
        e.datetime
    }
    
    pub fn last_date(id: i32, conn: &SqliteConnection) -> NaiveDateTime {
        let e = events::table
            .filter(events::course_id.eq(id))
            .order(events::datetime.desc())
            .first::<Event>(conn)
            .expect("Error loading course users");
        e.datetime
    }
}
