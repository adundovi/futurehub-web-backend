use rocket_contrib::json::Json;
use rocket::{
    Route,
};
use crate::db;

pub fn get_routes() -> Vec<Route> {
    routes![
        get,
        get_by_id,
        get_by_slug,
    ]
}

#[derive(Serialize)]
pub struct CategoryAttribs {
    pub title: String,
    pub slug: String,
    pub icon: Option<String>,
    pub description: Option<String>,
}

#[derive(Serialize)]
pub struct CategoryWrapper {
    pub id: i32,
    pub r#type: String,
    pub attributes: CategoryAttribs,
}

#[derive(Serialize)]
pub struct JsonApiResponse {
    data: Vec<CategoryWrapper>,
}

#[derive(Serialize)]
pub struct JsonSingleApiResponse {
    data: CategoryWrapper,
}

#[get("/category")]
pub fn get(conn: db::MainDbConn) -> Json<JsonApiResponse> {
    let mut response = JsonApiResponse { data: vec![], };

    for p in db::models::category::query(&conn) {
        let attribs = CategoryAttribs{
                        title: p.title,
                        slug: p.slug,
                        icon: p.icon,
                        description: p.description,
                    };
        let postw = CategoryWrapper{ id: p.id, r#type: "category".to_string(), attributes: attribs };
        response.data.push(postw);
    }
    Json(response)
}

#[get("/category/<id>")]
pub fn get_by_id(conn: db::MainDbConn, id: i32) -> Option<Json<JsonSingleApiResponse>> {

    let p = db::models::category::get(&conn, id).ok()?;
    let attribs = CategoryAttribs{
         title: p.title,
         slug: p.slug,
         icon: p.icon,
         description: p.description,
    };

    Some(Json(JsonSingleApiResponse{
        data: CategoryWrapper{
            id: p.id,
            r#type: "category".to_string(),
            attributes: attribs },
    }))
}

#[get("/category/<slug>", rank = 2)]
pub fn get_by_slug(conn: db::MainDbConn, slug: String) -> Option<Json<JsonSingleApiResponse>> {

    let p = db::models::category::get_by_slug(&conn, slug).ok()?;
    let attribs = CategoryAttribs{
         title: p.title,
         slug: p.slug,
         icon: p.icon,
         description: p.description,
    };

    Some(Json(JsonSingleApiResponse{
        data: CategoryWrapper{
            id: p.id,
            r#type: "category".to_string(),
            attributes: attribs },
    }))
}
