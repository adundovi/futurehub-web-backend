use rocket_contrib::json::Json;
use rocket::response::status;
use rocket::http::Status;
use crate::db;
use crate::db::models::course::{Course, CourseAttribs};
use crate::db::models::event::{Event, EventAttribs};
use super::response::{ResponseWithStatus, Response};
use super::response::{Data, SingleItem, VectorItems, ItemWrapper, Attribs};

use chrono::NaiveDateTime;

fn response_courses(courses: Vec<Course>) -> ResponseWithStatus {
    let mut items = VectorItems::new();
    
    for c in courses {
        let attribs = CourseAttribs{
                        title: c.title,
                        code: c.code,
                        description: c.description,
                        creation_date: c.creation_date,
                        lecturer: c.lecturer,
                        organizer: c.organizer,
                        lectures: c.lectures,
                        lecture_duration: c.lecture_duration,
                        students: c.students,
                        max_students: c.max_students,
                        finished: c.finished,
                    };
        let w = ItemWrapper::new(c.id, "course", Attribs::CourseAttribs(attribs));
        items.push(w);
    }
    
    Data::Vector(items).get_response()
}

#[get("/courses")]
pub fn get(conn: db::MainDbConn) -> status::Custom<Json<Response>> {
    let courses = Course::get_all_published(&conn).unwrap();
    let response = response_courses(courses);
    status::Custom(
        Status::from_code(response.status_code).unwrap(),
        Json(response.response),
    )
}

fn response_course(c: Course) -> ResponseWithStatus {
    
    let attribs = CourseAttribs{
         title: c.title,
         code: c.code,
         description: c.description,
         creation_date: c.creation_date,
         lecturer: c.lecturer,
         organizer: c.organizer,
         lectures: c.lectures,
         lecture_duration: c.lecture_duration,
         students: c.students,
         max_students: c.max_students,
         finished: c.finished,
    };
    
    let item = SingleItem::new(
        ItemWrapper::new(c.id, "course", Attribs::CourseAttribs(attribs)),
        None);
    
    Data::Single(item).get_response()
}

#[get("/courses/<id>")]
pub fn get_by_id(conn: db::MainDbConn, id: i32) -> status::Custom<Json<Response>> {
    let p = Course::get_published(&conn, id).unwrap();
    let response = response_course(p);
    status::Custom(
        Status::from_code(response.status_code).unwrap(),
        Json(response.response),
    )
}

fn response_course_with_events(c: Course, events: Vec<Event>) -> ResponseWithStatus {
    
    let attribs = CourseAttribs{
         title: c.title,
         code: c.code.clone(),
         description: c.description,
         creation_date: c.creation_date,
         lecturer: c.lecturer,
         organizer: c.organizer,
         lectures: c.lectures,
         lecture_duration: c.lecture_duration,
         students: c.students,
         max_students: c.max_students,
         finished: c.finished,
    };
    
    let mut includes: Vec<ItemWrapper> = vec![];

    for e in events {
        let attribs = EventAttribs{
            title: e.title,
            body: e.body,
            place: e.place,
            datetime: e.datetime,
            audience: e.audience,
            status: e.status,
            course_code: Some(c.code.clone())};
        let eventw = ItemWrapper::new(e.id, "event", Attribs::EventAttribs(attribs));
        includes.push(eventw);
    }
    
    let item = SingleItem::new(
        ItemWrapper::new(
            c.id, "course", Attribs::CourseAttribs(attribs)
            ),
        Some(includes)
    );

    Data::Single(item).get_response()
}

#[derive(FromFormValue)]
pub enum CourseInclude {
    Events,
}

#[get("/courses/<code>?<include>", rank = 2)]
pub fn get_by_code(conn: db::MainDbConn, code: String,
                   include: Option<CourseInclude>) -> status::Custom<Json<Response>> {
    let p = Course::get_published_by_code(&conn, &code).unwrap();
    
    let response = match include {
        None => response_course(p),
        Some(inc) => match inc {
            CourseInclude::Events => {
                let events = Course::list_events(&conn, p.id);
                response_course_with_events(p, events)
            }
        }
    };
    
    status::Custom(
        Status::from_code(response.status_code).unwrap(),
        Json(response.response),
    )
}

/*
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
         code: p.code.clone(),
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
            status: e.status,
            course_code: Some(p.code.clone())};
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

*/
