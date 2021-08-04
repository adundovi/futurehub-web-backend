use crate::db::models::user::UserAttribs;
use super::response::{Response, ResponseWithStatus};
use crate::consts::messages;
use rocket::http::Status;

#[derive(Serialize)]
pub struct JsonApiResponse {
    pub data: Vec<ItemWrapper>,
}

#[derive(Serialize)]
pub struct ItemWrapper {
    pub id: i32,
    pub r#type: String,
    pub attributes: Attribs,
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum Attribs {
    UserAttribs(UserAttribs)
}

impl JsonApiResponse {
    pub fn get_response(self) -> ResponseWithStatus {
        ResponseWithStatus {
            status_code: Status::Ok.code,
            response: Response {
                message: String::from(messages::MESSAGE_OK),
                data: serde_json::to_value(self.data).unwrap(),
            },
        }
    }
}

impl ItemWrapper {
    pub fn new(i: i32, t: &str, a: Attribs) -> Self {
        Self{ id: i, r#type: t.to_string(), attributes: a }
    }
}
