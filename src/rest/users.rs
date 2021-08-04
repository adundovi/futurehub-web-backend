use rocket_contrib::json::Json;
use rocket::response::status;
use rocket::http::Status;
use super::response::{Response, ResponseWithStatus};
use crate::db::{
    MainDbConn,
    models::user::{
        User,
        UserAttribs,
        NewUser,
    },
};
use crate::consts::messages;
use super::jwt::UserToken;
use super::common::{JsonApiResponse, ItemWrapper, Attribs};

use crate::db::model_traits::Queries;

fn response_users(users: Vec<User>) -> ResponseWithStatus {
    let mut response = JsonApiResponse { data: vec![], };
    
    for u in users {
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
        let w = ItemWrapper::new(u.id, "user", Attribs::UserAttribs(attribs));
        response.data.push(w);
    }
    
    response.get_response()
}

#[get("/users")]
pub fn get(
    token: Result<UserToken, status::Custom<Json<Response>>>,
    conn: MainDbConn) -> status::Custom<Json<Response>> {
    if let Err(e) = token {
        return e;
    }
    let users = User::get_all(&conn).unwrap();
    let response = response_users(users);
    status::Custom(
        Status::from_code(response.status_code).unwrap(),
        Json(response.response),
    )
}

#[options("/users")]
pub fn option<'a>() -> rocket::Response<'a> {
    let mut res = rocket::Response::new();
    res.set_status(Status::new(200, "No Content"));
    res
}

#[post("/users", format = "json", data = "<user>")]
pub fn post(
    user: Json<NewUser>,
    token: Result<UserToken, status::Custom<Json<Response>>>,
    conn: MainDbConn) -> status::Custom<Json<Response>> {
    if let Err(e) = token {
        return e;
    }
    //let t = token.unwrap();
    //TODO: group permission for this 
    User::create_full(&conn, user.into_inner());

    let response = ResponseWithStatus {
            status_code: Status::Ok.code,
            response: Response {
                message: String::from(messages::MESSAGE_SENT_SUCCESS),
                data: serde_json::to_value("").unwrap(),
            },
    };

    status::Custom(
        Status::from_code(response.status_code).unwrap(),
        Json(response.response),
    )
}

#[options("/users/<_id>")]
pub fn option_by_id<'a>(_id: i32) -> rocket::Response<'a> {
    let mut res = rocket::Response::new();
    res.set_status(Status::new(200, "No Content"));
    res
}

#[delete("/users/<id>")]
pub fn delete_by_id(
    conn: MainDbConn,
    token: Result<UserToken, status::Custom<Json<Response>>>,
    id: i32) -> status::Custom<Json<Response>> {
    if let Err(e) = token {
        return e;
    }
    User::remove(&conn, id);

    let response = ResponseWithStatus {
            status_code: Status::Ok.code,
            response: Response {
                message: String::from(messages::MESSAGE_SENT_SUCCESS),
                data: serde_json::to_value("").unwrap(),
            },
    };

    status::Custom(
        Status::from_code(response.status_code).unwrap(),
        Json(response.response),
    )
}

#[put("/users/<id>", format = "json", data = "<user>")]
pub fn put_by_id(
    conn: MainDbConn,
    token: Result<UserToken, status::Custom<Json<Response>>>,
    id: i32,
    user: Json<UserAttribs>) -> status::Custom<Json<Response>> {
    if let Err(e) = token {
        return e;
    }
    
    let item = User::get(&conn, id).expect("Id not found");
    let mut updated_item = item.clone();
    
    updated_item.username = user.username.clone();
    updated_item.email = user.email.clone();
    updated_item.oib = user.oib.clone();
    updated_item.name = user.name.clone();
    updated_item.surname = user.surname.clone();
    updated_item.address = user.address.clone();
    updated_item.phone = user.phone.clone();
    updated_item.birthday = user.birthday.clone();
    updated_item.gender = user.gender.clone();
    
    User::update(&conn, &updated_item);

    let response = ResponseWithStatus {
            status_code: Status::Ok.code,
            response: Response {
                message: String::from(messages::MESSAGE_SENT_SUCCESS),
                data: serde_json::to_value("").unwrap(),
            },
    };

    status::Custom(
        Status::from_code(response.status_code).unwrap(),
        Json(response.response),
    )
}
