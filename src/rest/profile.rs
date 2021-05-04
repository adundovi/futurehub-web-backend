use rocket_contrib::json::Json;
use serde_json::json;
use rocket::response::status;
use rocket::http::Status;
use super::response::{Response, ResponseWithStatus};
use jwt::UserToken;
use crate::db;
use crate::rest::jwt;
use crate::consts::messages;

fn get_profile(username: &str, conn: db::MainDbConn) -> ResponseWithStatus {
    let user = db::models::User::get_user_by_username(username, &conn).unwrap();
    ResponseWithStatus {
        status_code: Status::Ok.code,
        response: Response {
            message: String::from(messages::MESSAGE_OK),
            data: serde_json::to_value(
                user
                ).unwrap(),
        },
    }
}

#[get("/profile")]
pub fn get_info(
    token: Result<UserToken, status::Custom<Json<Response>>>,
    conn: db::MainDbConn,
) -> status::Custom<Json<Response>> {
    if let Err(e) = token {
        return e;
    }
    let t = token.unwrap();
    let response = get_profile(&t.user, conn);
    status::Custom(
        Status::from_code(response.status_code).unwrap(),
        Json(response.response),
    )
}
