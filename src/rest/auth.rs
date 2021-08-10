use rocket_contrib::json::Json;
use serde_json::json;
use rocket::http::Status;
use super::jwt::UserToken;
use super::response;
use super::response::{Data, Message, Response, ResponseWithStatus};
use crate::db;
use crate::rest::jwt;
use crate::consts::messages;

#[options("/auth/ping")]
pub fn option_ping<'a>() -> rocket::Response<'a> {
    let mut res = rocket::Response::new();
    res.set_status(Status::new(200, "No Content"));
    res
}

#[get("/auth/ping")]
pub fn ping(token: Result<UserToken, ResponseWithStatus>) -> ResponseWithStatus {
    let message: &str = if let Err(_e) = token { "unauthenticated" } else { "authenticated" };
        
    ResponseWithStatus {
        status: Status::Ok,
        response: Response::Message(
                        Message::new(message.to_string())
                  )
    }
}

pub fn login_user(conn: &db::MainDbConn, login: db::models::user::LoginData) -> ResponseWithStatus {
    if let Some(result) = db::models::user::User::login(&conn, login) {
        ResponseWithStatus {
            status: Status::Ok,
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
            status: Status::BadRequest,
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
pub fn post_login(login: Json<db::models::user::LoginData>, conn: db::MainDbConn) -> ResponseWithStatus {
    login_user(&conn, login.0)
}

pub fn signup(conn: &db::MainDbConn, user: db::models::user::UserDTO) -> ResponseWithStatus {
    if db::models::user::User::create_with_password(conn, user) {
        ResponseWithStatus {
            status: Status::Ok,
            response: Response::Message(
                response::Message::new(
                    String::from(messages::MESSAGE_SIGNUP_SUCCESS)
            )),
        }
    } else {
        ResponseWithStatus {
            status: Status::BadRequest,
            response: Response::Message(
                response::Message::new(
                    String::from(messages::MESSAGE_SIGNUP_FAILED)
                )),
        }
    }
}

#[post("/auth/signup", format = "json", data = "<user>")]
pub fn post_signup(user: Json<db::models::user::UserDTO>, conn: db::MainDbConn) -> ResponseWithStatus {
    signup(&conn, user.0)
}
