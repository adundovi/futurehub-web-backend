use rocket_contrib::json::Json;
use serde_json::json;
use rocket::response::status;
use rocket::http::Status;
use super::response;
use super::response::{Data, Response, ResponseWithStatus};
use crate::db;
use crate::rest::jwt;
use crate::consts::messages;

pub fn login_user(conn: &db::MainDbConn, login: db::models::user::LoginData) -> ResponseWithStatus {
    if let Some(result) = db::models::user::User::login(&conn, login) {
        ResponseWithStatus {
            status_code: Status::Ok.code,
            //message: String::from(messages::MESSAGE_LOGIN_SUCCESS),
            response: Response::Data(
                Data::Json(
                        json!({
                            "data": {
                                "token": jwt::generate_token(result),
                                "type": "Bearer"
                            }
                        })
                    )
                )
        }
    } else {
        ResponseWithStatus {
            status_code: Status::BadRequest.code,
            response: Response::Message(
                    response::Message::new(
                        String::from(messages::MESSAGE_LOGIN_FAILED)
                        )
                )
        }
    }
}

#[options("/auth/login")]
pub fn option_login<'a>() -> rocket::Response<'a> {
    let mut res = rocket::Response::new();
    res.set_status(Status::new(200, "No Content"));
    res
}

#[post("/auth/login", format = "json", data = "<login>")]
pub fn post_login(login: Json<db::models::user::LoginData>, conn: db::MainDbConn) -> status::Custom<Json<Response>> {
    let response = login_user(&conn, login.0);
    status::Custom(
        Status::from_code(response.status_code).unwrap(),
        Json(response.response),
    )
}

pub fn signup(conn: &db::MainDbConn, user: db::models::user::UserDTO) -> ResponseWithStatus {
    if db::models::user::User::create_with_password(conn, user) {
        ResponseWithStatus {
            status_code: Status::Ok.code,
            response: Response::Message(
                response::Message::new(
                    String::from(messages::MESSAGE_SIGNUP_SUCCESS)
            )),
        }
    } else {
        ResponseWithStatus {
            status_code: Status::BadRequest.code,
            response: Response::Message(
                response::Message::new(
                    String::from(messages::MESSAGE_SIGNUP_FAILED)
                )),
        }
    }
}

#[post("/auth/signup", format = "json", data = "<user>")]
pub fn post_signup(user: Json<db::models::user::UserDTO>, conn: db::MainDbConn) -> status::Custom<Json<Response>> {
    let response = signup(&conn, user.0);
    status::Custom(
        Status::from_code(response.status_code).unwrap(),
        Json(response.response),
    )
}
