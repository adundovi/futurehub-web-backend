use serde_json::Value;

// Response Code
// https://developer.mozilla.org/en-US/docs/Web/HTTP/Status
// https://api.rocket.rs/v0.4/rocket/response/status/index.html
// https://tools.ietf.org/html/rfc7807
// https://www.baeldung.com/rest-api-error-handling-best-practices
// https://api.rocket.rs/v0.4/rocket/response/struct.Response.html

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub message: String,
    pub data: Value,
}

#[derive(Debug)]
pub struct ResponseWithStatus {
    pub status_code: u16,
    pub response: Response,
}
