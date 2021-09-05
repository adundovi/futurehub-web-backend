use crate::{
    db,
    db::models::post::PostAttribs
};
use super::response::ResponseWithStatus;
use super::response::{SingleItem, VectorItems, Data, ItemWrapper, Attribs};

use rocket::{
    Route,
};

pub fn get_routes() -> Vec<Route> {
    routes![
        get,
        get_by_id,
        get_by_slug,
    ]
}

#[get("/posts")]
pub fn get(conn: db::MainDbConn) -> ResponseWithStatus {
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
    
    Data::Vector(items).to_response()
}

#[get("/posts/<id>")]
pub fn get_by_id(conn: db::MainDbConn, id: i32) -> ResponseWithStatus {

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

    Data::Single(item).to_response()
}

#[get("/posts/<slug>", rank = 2)]
pub fn get_by_slug(conn: db::MainDbConn, slug: String) -> ResponseWithStatus {

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

    Data::Single(item).to_response()
}
