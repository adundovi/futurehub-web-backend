use rocket_contrib::json::Json;
use crate::db;
use chrono::NaiveDateTime;

#[derive(Serialize)]
pub struct PostAttribs {
    pub title: String,
    pub slug: String,
    pub datetime: NaiveDateTime,
    pub body: Option<String>,
}

#[derive(Serialize)]
pub struct PostWrapper {
    pub id: i32,
    pub r#type: String,
    pub attributes: PostAttribs,
}

#[derive(Serialize)]
pub struct JsonApiResponse {
    data: Vec<PostWrapper>,
}

#[derive(Serialize)]
pub struct JsonSingleApiResponse {
    data: PostWrapper,
}

#[get("/posts")]
pub fn get(conn: db::MainDbConn) -> Json<JsonApiResponse> {
    let mut response = JsonApiResponse { data: vec![], };

    for p in db::post::query_published(&conn) {
        let attribs = PostAttribs{
            title: p.title,
            slug: p.slug,
            body: p.body,
            datetime: p.datetime,
        };
        let postw = PostWrapper{ id: p.id, r#type: "post".to_string(), attributes: attribs };
        response.data.push(postw);
    }

    Json(response)
}

#[get("/posts/<id>")]
pub fn get_by_id(conn: db::MainDbConn, id: i32) -> Option<Json<JsonSingleApiResponse>> {

    let p = db::post::get(&conn, id).ok()?;
    let attribs = PostAttribs{
         title: p.title,
         slug: p.slug,
         body: p.body,
         datetime: p.datetime,
    };

    Some(Json(JsonSingleApiResponse{
        data: PostWrapper{
            id: p.id,
            r#type: "post".to_string(),
            attributes: attribs },
    }))
}

#[get("/posts/<slug>", rank = 2)]
pub fn get_by_slug(conn: db::MainDbConn, slug: String) -> Option<Json<JsonSingleApiResponse>> {

    let p = db::post::get_by_slug(&conn, slug).ok()?;
    let attribs = PostAttribs{
         title: p.title,
         slug: p.slug,
         body: p.body,
         datetime: p.datetime,
    };

    Some(Json(JsonSingleApiResponse{
        data: PostWrapper{
            id: p.id,
            r#type: "post".to_string(),
            attributes: attribs },
    }))
}
