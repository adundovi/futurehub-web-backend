use rocket_contrib::json::Json;
use serde_json::json;
use rocket::response::status;
use rocket::http::Status;
use super::response::{Data, Response, ResponseWithStatus};
use jwt::UserToken;
use crate::db;
use crate::rest::jwt;
use crate::consts::messages;

fn get_profile(conn: &db::MainDbConn, username: &str) -> ResponseWithStatus {
    let user = db::models::user::User::get_user_by_username(&conn, username).unwrap();
    ResponseWithStatus {
        status_code: Status::Ok.code,
        response: Response::Data(
            Data::Json(
                json!({
                    "data": serde_json::to_value(user).unwrap()
                }))
            )
            //message: String::from(messages::MESSAGE_OK),
    }
}

#[options("/profile")]
pub fn option_info<'a>() -> rocket::Response<'a> {
    let mut res = rocket::Response::new();
    res.set_status(Status::new(200, "No Content"));
    res
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
    let response = get_profile(&conn, &t.user);
    status::Custom(
        Status::from_code(response.status_code).unwrap(),
        Json(response.response),
    )
}
