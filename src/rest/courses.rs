use rocket_contrib::json::Json;
use crate::db;
use crate::db::models::course::Course;
use crate::db::models::event::Event;
use super::events::{EventAttribs, EventWrapper};

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
    included: Option<Vec<EventWrapper>>
}

#[get("/courses")]
pub fn get(conn: db::MainDbConn) -> Option<Json<JsonApiResponse>> {
    let mut response = JsonApiResponse { data: vec![], };

    for p in Course::get_all_published(&conn).ok()? {
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
    Some(Json(response))
}

fn course2json(p: Course) -> Option<Json<JsonSingleApiResponse>> {
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
        included: None,
    }))
}

fn course2json_with_events(p: Course, events: Vec<Event>) -> Option<Json<JsonSingleApiResponse>> {
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

    let mut includes: Vec<EventWrapper> = vec![];

    for e in events {
        let attribs = EventAttribs{
            title: e.title,
            body: e.body,
            place: e.place,
            datetime: e.datetime,
            audience: e.audience,
            status: e.status };
        let eventw = EventWrapper{ id: e.id, r#type: "event".to_string(), attributes: attribs };
        includes.push(eventw);
    }

    Some(Json(JsonSingleApiResponse{
        data: CourseWrapper{
            id: p.id,
            r#type: "course".to_string(),
            attributes: attribs },
        included: Some(includes),
    }))
}

#[get("/courses/<id>")]
pub fn get_by_id(conn: db::MainDbConn, id: i32) -> Option<Json<JsonSingleApiResponse>> {
    let p = Course::get_published(&conn, id).ok()?;
    course2json(p)
}

#[derive(FromFormValue)]
pub enum CourseInclude {
    Events,
}

#[get("/courses/<code>?<include>", rank = 2)]
pub fn get_by_code(conn: db::MainDbConn, code: String, include: Option<CourseInclude>) -> Option<Json<JsonSingleApiResponse>> {
    let p = Course::get_published_by_code(&conn, &code).ok()?;
    match include {
        None => course2json(p),
        Some(i) => match i {
            CourseInclude::Events => {
                let events = Course::list_events(&conn, p.id);
                course2json_with_events(p, events)
            }
        }
    }
}
