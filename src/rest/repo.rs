use crate::db::{
    MainDbConn,
    models::repo_items::{
        self,
        RepoItem,
        RepoAttribs,
    }
};
use super::response::{Data, SingleItem, VectorItems, ItemWrapper, Attribs, Message, Response, ResponseWithStatus};
use chrono::NaiveDateTime;
use rocket::response::NamedFile;

#[get("/repo?<category>")]
pub fn get(conn: MainDbConn, category: Option<String>) -> ResponseWithStatus {
    let mut items = VectorItems::new();

    match category {
        None => {
            for p in repo_items::query_published(&conn) {
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
                    let item = ItemWrapper::new(p.id, "file", Attribs::RepoAttribs(attribs));
                    items.push(item);
            }
        },
        Some(c) =>
            for p in repo_items::query_published_by_category(&conn, &c) {
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
                    let item = ItemWrapper::new(p.id, "file", Attribs::RepoAttribs(attribs));
                    items.push(item);
            }
    }
    
    Data::Vector(items).to_response()
}

#[get("/repo/<id>", rank = 1)]
pub fn get_by_id(conn: MainDbConn, id: i32) -> ResponseWithStatus {

    let p = repo_items::get(&conn, id).unwrap();
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
    
    let item = SingleItem::new(
        ItemWrapper::new(p.id, "file", Attribs::RepoAttribs(attribs)),
        None);
    
    Data::Single(item).to_response()
}

#[get("/repo/<slug>", rank = 2)]
pub fn get_by_slug(conn: MainDbConn, slug: String) -> ResponseWithStatus {

    let p = repo_items::get_by_slug(&conn, slug).unwrap();
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
    
    let item = SingleItem::new(
        ItemWrapper::new(p.id, "file", Attribs::RepoAttribs(attribs)),
        None);
    
    Data::Single(item).to_response()
}

/* see also X-Sendfile */
#[get("/repo/stream/<slug>")]
pub fn get_stream_by_slug(conn: MainDbConn, slug: String) -> Option<NamedFile> {
    let p = repo_items::get_by_slug(&conn, slug).ok()?;
    NamedFile::open(&p.filepath).ok() 
}
