use rocket_contrib::json::Json;
use super::super::db;
use chrono::NaiveDateTime;

#[derive(Serialize)]
pub struct EventAttribs {
    pub title: String,
    pub datetime: NaiveDateTime,
    pub body: Option<String>,
    pub place: Option<String>,
    pub audience: Option<String>,
}

#[derive(Serialize)]
pub struct EventWrapper {
    pub id: i32,
    pub r#type: String,
    pub attributes: EventAttribs,
}

#[derive(Serialize)]
pub struct JsonApiResponse {
    data: Vec<EventWrapper>,
}


#[get("/api/events")]
pub fn events_get(conn: db::MainDbConn) -> Json<JsonApiResponse> {
    let mut response = JsonApiResponse { data: vec![], };

    for event in db::event::query(&conn) {
        let attribs = EventAttribs{
            title: event.title,
            body: event.body,
            place: event.place,
            datetime: event.datetime,
            audience: event.audience };
        let eventw = EventWrapper{ id: event.id, r#type: "event".to_string(), attributes: attribs };
        response.data.push(eventw);
    }

    Json(response)
}

#[get("/api/events/newest")]
pub fn newest_events_get(conn: db::MainDbConn) -> Json<JsonApiResponse> {
    let mut response = JsonApiResponse { data: vec![], };

    let mut events = db::event::query_newest(&conn, 3);
    events.reverse();
    for event in events {
        let attribs = EventAttribs{
            title: event.title,
            body: event.body,
            place: event.place,
            datetime: event.datetime,
            audience: event.audience };
        let eventw = EventWrapper{ id: event.id, r#type: "event".to_string(), attributes: attribs };
        response.data.push(eventw);
    }

    Json(response)
}
