use crate::db::models::{
    user::UserAttribs,
    course::CourseAttribs,
    event::EventAttribs,
    post::PostAttribs,
};
use crate::consts::messages;
use rocket::http::Status;
use serde_json::Value as JsonValue;

// Response Code
// https://developer.mozilla.org/en-US/docs/Web/HTTP/Status
// https://api.rocket.rs/v0.4/rocket/response/status/index.html
// https://tools.ietf.org/html/rfc7807
// https://www.baeldung.com/rest-api-error-handling-best-practices
// https://api.rocket.rs/v0.4/rocket/response/struct.Response.html


#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Attribs {
    UserAttribs(UserAttribs),
    CourseAttribs(CourseAttribs),
    EventAttribs(EventAttribs),
    PostAttribs(PostAttribs),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemWrapper {
    pub id: i32,
    pub r#type: String,
    pub attributes: Attribs,
}

impl ItemWrapper {
    pub fn new(i: i32, t: &str, a: Attribs) -> Self {
        Self{ id: i, r#type: t.to_string(), attributes: a }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VectorItems {
    pub data: Vec<ItemWrapper>,
}

impl VectorItems {
    pub fn new() -> VectorItems {
        VectorItems{ data: vec![] }
    }
    pub fn push(&mut self, item: ItemWrapper) -> () {
        self.data.push(item);
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SingleItem {
    pub data: ItemWrapper,
    pub included: Option<Vec<ItemWrapper>>
}

impl SingleItem {
    pub fn new(item: ItemWrapper, inc: Option<Vec<ItemWrapper>>) -> SingleItem {
        SingleItem{
            data: item,
            included: inc
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Data {
    Single(SingleItem),
    Vector(VectorItems),
    Json(JsonValue),
}

impl Data {
    pub fn get_response(self) -> ResponseWithStatus {
        ResponseWithStatus {
            status_code: Status::Ok.code,
            response: Response::Data(self)
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub message: String,
}

impl Message {
    pub fn new(s: String) -> Message {
        Message{ message: s }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Error {
    pub errors: JsonValue,
}

impl Error {
    pub fn new(jv: JsonValue) -> Error {
        Error{ errors: jv }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Response {
    Message(Message),
    Error(Error),
    Data(Data),
}

#[derive(Debug)]
pub struct ResponseWithStatus {
    pub status_code: u16,
    pub response: Response,
}
