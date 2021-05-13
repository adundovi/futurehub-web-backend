use rocket_contrib::json::Json;
use crate::db;
use crate::db::model_traits::Queries;
use chrono::NaiveDateTime;

#[derive(Serialize)]
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
}

#[derive(Serialize)]
pub struct CourseWrapper {
    pub id: i32,
    pub r#type: String,
    pub attributes: CourseAttribs,
}

#[derive(Serialize)]
pub struct JsonApiResponse {
    data: Vec<CourseWrapper>,
}

#[derive(Serialize)]
pub struct JsonSingleApiResponse {
    data: CourseWrapper,
}

#[get("/courses")]
pub fn get(conn: db::MainDbConn) -> Json<JsonApiResponse> {
    let mut response = JsonApiResponse { data: vec![], };

    for p in db::models::course::Course::get_all_published(&conn).expect("No found") {
        let attribs = CourseAttribs{
                        title: p.title,
                        code: p.code,
                        description: p.description,
                        creation_date: p.creation_date,
                        lecturer: p.lecturer,
                        organizer: p.organizer,
                        lectures: p.lectures,
                        lecture_duration: p.lecture_duration,
                        students: p.students,
                        max_students: p.max_students,
                        finished: p.finished,
                    };
        let postw = CourseWrapper{ id: p.id, r#type: "course".to_string(), attributes: attribs };
        response.data.push(postw);
    }
    Json(response)
}

#[get("/courses/<id>")]
pub fn get_by_id(conn: db::MainDbConn, id: i32) -> Option<Json<JsonSingleApiResponse>> {

    let p = db::models::course::Course::get_published(&conn, id).ok()?;
    let attribs = CourseAttribs{
         title: p.title,
         code: p.code,
         description: p.description,
         creation_date: p.creation_date,
         lecturer: p.lecturer,
         organizer: p.organizer,
         lectures: p.lectures,
         lecture_duration: p.lecture_duration,
         students: p.students,
         max_students: p.max_students,
         finished: p.finished,
    };

    Some(Json(JsonSingleApiResponse{
        data: CourseWrapper{
            id: p.id,
            r#type: "course".to_string(),
            attributes: attribs },
    }))
}

#[get("/courses/<code>", rank = 2)]
pub fn get_by_code(conn: db::MainDbConn, code: String) -> Option<Json<JsonSingleApiResponse>> {

    let p = db::models::course::Course::get_published_by_code(&conn, &code).ok()?;
    let attribs = CourseAttribs{
         title: p.title,
         code: p.code,
         description: p.description,
         creation_date: p.creation_date,
         lecturer: p.lecturer,
         organizer: p.organizer,
         lectures: p.lectures,
         lecture_duration: p.lecture_duration,
         students: p.students,
         max_students: p.max_students,
         finished: p.finished,
    };

    Some(Json(JsonSingleApiResponse{
        data: CourseWrapper{
            id: p.id,
            r#type: "course".to_string(),
            attributes: attribs },
    }))
}
