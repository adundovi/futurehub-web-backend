use rocket_contrib::json::Json;
use serde_json::json;
use rocket::response::status;
use rocket::http::Status;
use super::response::{Response, ResponseWithStatus};
use crate::db;
use crate::rest::jwt;
use crate::consts::messages;

pub fn login_user(login: db::models::LoginData, conn: db::MainDbConn) -> ResponseWithStatus {
    if let Some(result) = db::models::User::login(login, &conn) {
        ResponseWithStatus {
            status_code: Status::Ok.code,
            response: Response {
                message: String::from(messages::MESSAGE_LOGIN_SUCCESS),
                data: serde_json::to_value(json!({ "token": jwt::generate_token(result), "type": "Bearer" }))
                    .unwrap(),
            },
        }
    } else {
        ResponseWithStatus {
            status_code: Status::BadRequest.code,
            response: Response {
                message: String::from(messages::MESSAGE_LOGIN_FAILED),
                data: serde_json::to_value("").unwrap(),
            },
        }
    }
}

#[post("/login", format = "json", data = "<login>")]
pub fn process_login(login: Json<db::models::LoginData>, conn: db::MainDbConn) -> status::Custom<Json<Response>> {
    let response = login_user(login.0, conn);
    status::Custom(
        Status::from_code(response.status_code).unwrap(),
        Json(response.response),
    )
}

/*
//#[derive(Insertable, Serialize, Deserialize)]
//#[table_name = "users"]
pub struct UserDTO {
    pub username: String,
    pub email: String,
    pub password: String,
}
*/
/*
pub fn signup(user: UserDTO, conn: DbConn) -> ResponseWithStatus {
    if User::signup(user, &conn) {
        ResponseWithStatus {
            status_code: Status::Ok.code,
            response: Response {
                message: String::from(message_constants::MESSAGE_SIGNUP_SUCCESS),
                data: serde_json::to_value("").unwrap(),
            },
        }
    } else {
        ResponseWithStatus {
            status_code: Status::BadRequest.code,
            response: Response {
                message: String::from(message_constants::MESSAGE_SIGNUP_FAILED),
                data: serde_json::to_value("").unwrap(),
            },
        }
    }
}

#[post("/signup", format = "json", data = "<user>")]
pub fn signup(user: Json<UserDTO>, conn: db::MainDbConn) -> status::Custom<Json<Response>> {
    let response = account_service::signup(user.0, conn);
    status::Custom(
        Status::from_code(response.status_code).unwrap(),
        Json(response.response),
    )
}
*/
