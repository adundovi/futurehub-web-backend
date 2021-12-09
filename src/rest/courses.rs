use crate::db::{
    MainDbConn,
    models::{
        course::{NewCourse,
            Course,
            CourseAttribs,
            CourseUser},
        event::{Event, EventAttribs},
        user::{User, UserAttribs},
    },
};
use crate::db::model_traits::Queries;
use crate::consts::messages;
use super::response::{
    Response,
    Message,
    ResponseWithStatus,
    Data,
    SingleItem,
    VectorItems,
    ItemWrapper,
    Attribs
};
use rocket::{
    Route,
    http::Status,
};
use super::jwt::UserToken;
use rocket_contrib::json::Json;

pub fn get_routes() -> Vec<Route> {
    routes![
        get,
        get_by_id,
        get_by_code,
        get_by_id_events,
        get_by_id_participants,
        option,
        option_by_id_events,
        option_by_id_participants,
        post,
        option_by_id,
        delete_by_id,
        put_by_id,
    ]
}

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
                        published: c.published,
                    };
        let w = ItemWrapper::new(c.id, "course", Attribs::CourseAttribs(attribs));
        items.push(w);
    }
    
    Data::Vector(items).to_response()
}

#[get("/courses")]
pub fn get(conn: MainDbConn) -> ResponseWithStatus {
    let courses = Course::get_all_published(&conn).unwrap();
    response_courses(courses)
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
         published: c.published,
    };
    
    let item = SingleItem::new(
        ItemWrapper::new(c.id, "course", Attribs::CourseAttribs(attribs)),
        None);
    
    Data::Single(item).to_response()
}

#[get("/courses/<id>")]
pub fn get_by_id(conn: MainDbConn, id: i32) -> ResponseWithStatus {
    let p = Course::get_published(&conn, id).unwrap();
    response_course(p)
}

#[get("/courses/<code>", rank = 2)]
pub fn get_by_code(conn: MainDbConn, code: String) -> ResponseWithStatus {
    let p = Course::get_published_by_code(&conn, &code).unwrap();
    response_course(p)
}

fn response_course_with_events(c: Course, events: Vec<Event>) -> ResponseWithStatus {
    
    let mut items = VectorItems::new();

    for e in events {
        let attribs = EventAttribs{
            title: e.title,
            body: e.body,
            place: e.place,
            datetime: e.datetime,
            audience: e.audience,
            status: e.status,
            course_id: e.course_id,
            course_code: Some(c.code.clone())};
        let item = ItemWrapper::new(e.id, "event", Attribs::EventAttribs(attribs));
        items.push(item);
    }
    
    Data::Vector(items).to_response()
}

fn response_course_with_participants(
    _c: Course,
    participants: Vec<(User, CourseUser)>
    ) -> ResponseWithStatus {
    
    let mut items = VectorItems::new();

    for (u, _cu) in participants {
        let attribs = UserAttribs{
            username: u.username,
            email: u.email,
            login_session: u.login_session,
            oib: u.oib,
            name: u.name,
            surname: u.surname,
            address: u.address,
            phone: u.phone,
            gender: u.gender,
            birthday: u.birthday,
            creation_date: u.creation_date,
        };
        let u = ItemWrapper::new(u.id, "user", Attribs::UserAttribs(attribs));
        items.push(u);
    }
    
    Data::Vector(items).to_response()
}

#[options("/courses/<_id>/events")]
pub fn option_by_id_events<'a>(_id: i32) -> rocket::Response<'a> {
    let mut res = rocket::Response::new();
    res.set_status(Status::new(200, "No Content"));
    res
}

#[get("/courses/<id>/events")]
pub fn get_by_id_events(conn: MainDbConn, id: i32) -> ResponseWithStatus {
    let p = Course::get_published(&conn, id).unwrap();
    let events = Course::list_events(&conn, p.id);
    response_course_with_events(p, events)
}

#[options("/courses/<_id>/participants")]
pub fn option_by_id_participants<'a>(_id: i32) -> rocket::Response<'a> {
    let mut res = rocket::Response::new();
    res.set_status(Status::new(200, "No Content"));
    res
}

#[get("/courses/<id>/participants")]
pub fn get_by_id_participants(conn: MainDbConn, id: i32,
                        token: Result<UserToken, ResponseWithStatus>) -> ResponseWithStatus {
    if let Err(e) = token {
        return e
    }

    let p = Course::get_published(&conn, id).unwrap();
    let participants = Course::list_participants(&conn, p.id); 
    response_course_with_participants(p, participants)
}

#[options("/courses")]
pub fn option<'a>() -> rocket::Response<'a> {
    let mut res = rocket::Response::new();
    res.set_status(Status::new(200, "No Content"));
    res
}

#[post("/courses", format = "json", data = "<course>")]
pub fn post(
    conn: MainDbConn,
    course: Json<NewCourse>,
    token: Result<UserToken, ResponseWithStatus>) -> ResponseWithStatus {
    if let Err(e) = token {
        return e;
    }
    //let t = token.unwrap();
    //TODO: group permission for this 
    Course::create_full(&conn, course.into_inner());

    ResponseWithStatus {
            status: Status::Ok,
            response: Response::Message(
                Message::new(String::from(messages::MESSAGE_SENT_SUCCESS))
                )
    }
}

#[options("/courses/<_id>")]
pub fn option_by_id<'a>(_id: i32) -> rocket::Response<'a> {
    let mut res = rocket::Response::new();
    res.set_status(Status::new(200, "No Content"));
    res
}

#[delete("/courses/<id>")]
pub fn delete_by_id(
    conn: MainDbConn,
    token: Result<UserToken, ResponseWithStatus>,
    id: i32) -> ResponseWithStatus {
    if let Err(e) = token {
        return e;
    }
    let _ = Course::remove(&conn, id);

    ResponseWithStatus {
            status: Status::Ok,
            response: Response::Message(
                Message::new(String::from(messages::MESSAGE_SENT_SUCCESS))
                )
    }
}

#[put("/courses/<id>", format = "json", data = "<course>")]
pub fn put_by_id(
    conn: MainDbConn,
    token: Result<UserToken, ResponseWithStatus>,
    id: i32,
    course: Json<CourseAttribs>) -> ResponseWithStatus {
    if let Err(e) = token {
        return e;
    }
    
    let item = Course::get(&conn, id).expect("Id not found");
    let mut updated_item = item.clone();
    
    updated_item.title = course.title.clone();
    updated_item.code = course.code.clone();
    updated_item.description = course.description.clone();
    updated_item.creation_date = course.creation_date.clone();
    updated_item.lecturer = course.lecturer.clone();
    updated_item.organizer = course.organizer.clone();
    updated_item.lectures = course.lectures.clone();
    updated_item.lecture_duration = course.lecture_duration.clone();
    updated_item.students = course.students.clone();
    updated_item.max_students = course.max_students.clone();
    updated_item.finished = course.finished.clone();
    updated_item.published = course.published.clone();
    
    Course::update(&conn, &updated_item);

    ResponseWithStatus {
            status: Status::Ok,
            response: Response::Message(
                Message::new(String::from(messages::MESSAGE_SENT_SUCCESS))
                )
    }
}
