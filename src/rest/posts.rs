use rocket_contrib::json::Json;
use rocket::response::status;
use rocket::http::Status;
use crate::{
    db,
    db::models::post::PostAttribs
};
use chrono::NaiveDateTime;

use super::response::{Message, Response, ResponseWithStatus};
use super::response::{SingleItem, VectorItems, Data, ItemWrapper, Attribs};

#[get("/posts")]
pub fn get(conn: db::MainDbConn) -> status::Custom<Json<Response>> {
    let mut items = VectorItems::new();

    for p in db::models::post::get_all_published(&conn) {
        let attribs = PostAttribs{
            title: p.title,
            slug: p.slug,
            body: p.body,
            datetime: p.datetime,
        };
        let w = ItemWrapper::new(p.id, "post", Attribs::PostAttribs(attribs));
        items.push(w);
    }
    
    let r = Data::Vector(items).get_response();

    status::Custom(
        Status::from_code(r.status_code).unwrap(),
        Json(r.response),
    )
}

#[get("/posts/<id>")]
pub fn get_by_id(conn: db::MainDbConn, id: i32) -> status::Custom<Json<Response>> {

    let p = db::models::post::get(&conn, id).unwrap();
    let attribs = PostAttribs{
         title: p.title,
         slug: p.slug,
         body: p.body,
         datetime: p.datetime,
    };

    let item = SingleItem::new(
        ItemWrapper::new(
            p.id, "post", Attribs::PostAttribs(attribs)
            ),
        None
    );

    let r = Data::Single(item).get_response();
    
    status::Custom(
        Status::from_code(r.status_code).unwrap(),
        Json(r.response),
    )
}

#[get("/posts/<slug>", rank = 2)]
pub fn get_by_slug(conn: db::MainDbConn, slug: String) -> status::Custom<Json<Response>> {

    let p = db::models::post::get_by_slug(&conn, slug).unwrap();
    let attribs = PostAttribs{
         title: p.title,
         slug: p.slug,
         body: p.body,
         datetime: p.datetime,
    };

    let item = SingleItem::new(
        ItemWrapper::new(
            p.id, "post", Attribs::PostAttribs(attribs)
            ),
        None
    );

    let r = Data::Single(item).get_response();
    
    status::Custom(
        Status::from_code(r.status_code).unwrap(),
        Json(r.response),
    )
}
