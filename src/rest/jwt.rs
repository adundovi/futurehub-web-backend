use chrono::Utc;
use crate::db;
use crate::consts::messages;
use jsonwebtoken::errors::Result;
use jsonwebtoken::TokenData;
use jsonwebtoken::{Header, Validation};
use jsonwebtoken::{EncodingKey, DecodingKey};
use crate::rest::response::{Message, Response};
use crate::db::models::user::LoginInfo;
use rocket::http::Status;
use rocket::outcome::Outcome;
use rocket::request::{self, FromRequest, Request};
use super::response::ResponseWithStatus;

static ONE_WEEK: i64 = 60 * 60 * 24 * 7; // in seconds

#[derive(Debug, Serialize, Deserialize)]
pub struct UserToken {
    // issued at
    pub iat: i64,
    // expiration
    pub exp: i64,
    // data
    pub user: String,
    pub login_session: String,
}

impl<'a, 'r> FromRequest<'a, 'r> for UserToken {
    type Error = ResponseWithStatus;
    fn from_request(
        request: &'a Request<'r>,
    ) -> request::Outcome<Self, ResponseWithStatus> {
        let conn = request.guard::<db::MainDbConn>().unwrap();
        if let Some(authen_header) = request.headers().get_one("Authorization") {
            let authen_str = authen_header.to_string();
            if authen_str.starts_with("Bearer") {
                let token = authen_str[6..authen_str.len()].trim();
                if let Ok(token_data) = decode_token(token.to_string()) {
                    if verify_token(&conn, &token_data) {
                        return Outcome::Success(token_data.claims);
                    }
                } 
            }
        }

        Outcome::Failure((
            Status::BadRequest,
            ResponseWithStatus {
                status: Status::Unauthorized,
                response: Response::Message(
                            Message::new(String::from(messages::MESSAGE_INVALID_TOKEN))
                        )
            }
        ))
    }
}

pub fn generate_token(login: LoginInfo) -> String {
    let now = Utc::now().timestamp_nanos() / 1_000_000_000; // nanosecond -> second
    let payload = UserToken {
        iat: now,
        exp: now + ONE_WEEK,
        user: login.username,
        login_session: login.login_session,
    };

    let key = include_bytes!("../../secret.key");
    jsonwebtoken::encode(&Header::default(), &payload, &EncodingKey::from_secret(key)).unwrap()
}

fn decode_token(token: String) -> Result<TokenData<UserToken>> {
    let key = include_bytes!("../../secret.key");
    jsonwebtoken::decode::<UserToken>(&token, &DecodingKey::from_secret(key), &Validation::default())
}

fn verify_token(conn: &db::MainDbConn, token_data: &TokenData<UserToken>) -> bool {
    db::models::user::User::is_valid_login_session(conn, &token_data.claims)
}
