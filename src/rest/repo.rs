use rocket_contrib::json::Json;
use crate::db;
use chrono::NaiveDateTime;
use rocket::response::NamedFile;

#[derive(Serialize)]
pub struct RepoAttribs {
    pub title: String,
    pub slug: String,
    pub streampath: String,
    pub datetime: NaiveDateTime,
    pub description: Option<String>,
    pub filehash: Option<String>,
    pub filesize: Option<i64>,
    pub category_id: i32,
}

#[derive(Serialize)]
pub struct RepoWrapper {
    pub id: i32,
    pub r#type: String,
    pub attributes: RepoAttribs,
}

#[derive(Serialize)]
pub struct JsonApiResponse {
    data: Vec<RepoWrapper>,
}

#[derive(Serialize)]
pub struct JsonSingleApiResponse {
    data: RepoWrapper,
}

#[get("/repo?<category>")]
pub fn get(conn: db::MainDbConn, category: Option<String>) -> Json<JsonApiResponse> {
    let mut response = JsonApiResponse { data: vec![], };

    match category {
        None => {
            for p in db::models::repo_items::query_published(&conn) {
                    let attribs = RepoAttribs{
                        title: p.title,
                        slug: p.slug.clone(),
                        streampath: format!("/api/repo/stream/{}", &p.slug),
                        description: p.description,
                        datetime: p.datetime,
                        filehash: p.filehash,
                        filesize: p.filesize,
                        category_id: p.category_id,
                    };
                    let postw = RepoWrapper{ id: p.id, r#type: "file".to_string(), attributes: attribs };
                    response.data.push(postw);
            }
        },
        Some(c) =>
            for p in db::models::repo_items::query_published_by_category(&conn, &c) {
                    let attribs = RepoAttribs{
                        title: p.title,
                        slug: p.slug.clone(),
                        streampath: format!("/api/repo/stream/{}", &p.slug),
                        description: p.description,
                        datetime: p.datetime,
                        filehash: p.filehash,
                        filesize: p.filesize,
                        category_id: p.category_id,
                    };
                    let postw = RepoWrapper{ id: p.id, r#type: "file".to_string(), attributes: attribs };
                    response.data.push(postw);
            }
    }
    Json(response)
}

#[get("/repo/<id>", rank = 1)]
pub fn get_by_id(conn: db::MainDbConn, id: i32) -> Option<Json<JsonSingleApiResponse>> {

    let p = db::models::repo_items::get(&conn, id).ok()?;
    let attribs = RepoAttribs{
         title: p.title,
         slug: p.slug.clone(),
         streampath: format!("/api/repo/stream/{}", &p.slug),
         description: p.description,
         datetime: p.datetime,
         filehash: p.filehash,
         filesize: p.filesize,
         category_id: p.category_id,
    };

    Some(Json(JsonSingleApiResponse{
        data: RepoWrapper{
            id: p.id,
            r#type: "file".to_string(),
            attributes: attribs },
    }))
}

#[get("/repo/<slug>", rank = 2)]
pub fn get_by_slug(conn: db::MainDbConn, slug: String) -> Option<Json<JsonSingleApiResponse>> {

    let p = db::models::repo_items::get_by_slug(&conn, slug).ok()?;
    let attribs = RepoAttribs{
         title: p.title,
         slug: p.slug.clone(),
         streampath: format!("/api/repo/stream/{}", &p.slug),
         description: p.description,
         datetime: p.datetime,
         filehash: p.filehash,
         filesize: p.filesize,
         category_id: p.category_id,
    };

    Some(Json(JsonSingleApiResponse{
        data: RepoWrapper{
            id: p.id,
            r#type: "file".to_string(),
            attributes: attribs },
    }))
}

/* see also X-Sendfile */
#[get("/repo/stream/<slug>")]
pub fn get_stream_by_slug(conn: db::MainDbConn, slug: String) -> Option<NamedFile> {
    let p = db::models::repo_items::get_by_slug(&conn, slug).ok()?;
    NamedFile::open(&p.filepath).ok() 
}
