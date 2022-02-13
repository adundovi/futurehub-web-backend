use crate::db::{
    MainDbConn,
    models::repo_items::{
        self,
        RepoAttribs,
    }
};
use super::response::{Data, SingleItem, VectorItems, ItemWrapper, Attribs, ResponseWithStatus};
use rocket::{
    Route,
    response::NamedFile
};

use image::GenericImageView;
               
pub fn get_routes() -> Vec<Route> {
    routes![
        get,
        get_by_id,
        get_by_slug,
        get_stream_by_slug,
    ]
}

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
#[get("/repo/stream/<slug>?<size>")]
pub fn get_stream_by_slug(conn: MainDbConn, slug: String, size: Option<String>) -> Option<NamedFile> {
    let p = repo_items::get_by_slug(&conn, slug).ok()?;
    let path = std::path::Path::new(&p.filepath);
    let filename = path.file_stem().and_then(std::ffi::OsStr::to_str).unwrap();
    let ext = path.extension().and_then(std::ffi::OsStr::to_str).unwrap();
    let thumb_location = std::path::Path::new("./tmp/thumb");
    let allowed_exts = vec!["png", "jpg", "jpeg", "gif"];

    match size {
        None => NamedFile::open(&p.filepath).ok(),
        Some(s) => {

            if allowed_exts.contains(&ext) {
                return NamedFile::open(&p.filepath).ok();
            }

            let desired_size: Vec<&str> = s.split("x").collect();
            let (desired_w, desired_h): (u32, u32) = if desired_size.len() == 2 {
                let w = desired_size[0].parse::<u32>().unwrap_or(400);
                let h = desired_size[1].parse::<u32>().unwrap_or(300);
                (w, h)
            } else {
                (400, 300)
            };

            let thumbpath = thumb_location
                .join(format!("{}-{}x{}.{}",
                        &filename, desired_w, desired_h, &ext
                ));

            if thumbpath.exists() {
                NamedFile::open(&thumbpath).ok()
            } else {
                let mut img = image::open(&p.filepath).unwrap();
                let (w, h) = img.dimensions();
                let orig_ratio = w as f32 / (h as f32);
                let desired_ratio = desired_w as f32 / (desired_h as f32);
                let (smaller_w, smaller_h) = (
                    std::cmp::min(w, desired_w),
                    std::cmp::min(h, desired_h));
                let (new_w, new_h) = 
                    if orig_ratio == desired_ratio {
                        (smaller_w, smaller_h)
                    } else {
                        let w_from_h = orig_ratio * smaller_h as f32;
                        if w_from_h > smaller_w as f32 {
                            (smaller_w, (smaller_w as f32 / orig_ratio) as u32)
                        } else {
                            ((smaller_h as f32 * orig_ratio) as u32, smaller_h)
                        }
                    };
                println!("dim: {}x{}", new_w, new_h);
                let resized_img = image::imageops::resize(
                    &mut img, new_w, new_h, image::imageops::FilterType::Triangle);
                let _ = resized_img.save(&thumbpath);
                NamedFile::open(&thumbpath).ok()
            }
        }
    }
}
