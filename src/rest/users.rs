use rocket_contrib::json::Json;
use rocket::http::Status;
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
use super::response::{Message, ResponseWithStatus, Response};
use super::response::{Data, VectorItems, ItemWrapper, Attribs};

use crate::db::model_traits::Queries;

fn response_users(users: Vec<User>) -> ResponseWithStatus {
    let mut items = VectorItems::new();
    
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
        items.push(w);
    }
    
    Data::Vector(items).get_response()
}

#[get("/users")]
pub fn get(
    token: Result<UserToken, ResponseWithStatus>,
    conn: MainDbConn) -> ResponseWithStatus {
    if let Err(e) = token {
        return e;
    }
    let users = User::get_all(&conn).unwrap();
    response_users(users)
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
    token: Result<UserToken, ResponseWithStatus>,
    conn: MainDbConn) -> ResponseWithStatus {
    if let Err(e) = token {
        return e;
    }
    //let t = token.unwrap();
    //TODO: group permission for this 
    User::create_full(&conn, user.into_inner());

    ResponseWithStatus {
            status: Status::Ok,
            response: Response::Message(
                Message::new(String::from(messages::MESSAGE_SENT_SUCCESS))
                )
    }
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
    token: Result<UserToken, ResponseWithStatus>,
    id: i32) -> ResponseWithStatus {
    if let Err(e) = token {
        return e;
    }
    User::remove(&conn, id);

    ResponseWithStatus {
            status: Status::Ok,
            response: Response::Message(
                Message::new(String::from(messages::MESSAGE_SENT_SUCCESS))
                )
    }
}

#[put("/users/<id>", format = "json", data = "<user>")]
pub fn put_by_id(
    conn: MainDbConn,
    token: Result<UserToken, ResponseWithStatus>,
    id: i32,
    user: Json<UserAttribs>) -> ResponseWithStatus {
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

    ResponseWithStatus {
            status: Status::Ok,
            response: Response::Message(
                Message::new(String::from(messages::MESSAGE_SENT_SUCCESS))
                )
    }
}
