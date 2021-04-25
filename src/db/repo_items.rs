use diesel::{prelude::*, sqlite::SqliteConnection};
use chrono::prelude::*;
use std::path::Path;

use crate::db::models;
use crate::db::sqlite_schema::repo_items as repo_items;
use crate::db::sqlite_schema::categories as categories;
use crate::tools;

fn get_repopath() -> std::path::PathBuf {
    let c = crate::active_config();
    let c_repo = c.get_extra("repository").unwrap();
    let p = Path::new(
            c_repo["path"].as_str().unwrap()
    );
    p.to_path_buf()
}

fn prepare_file(filepath_: &String) -> (String, Option<String>) {
    let repopath = get_repopath();
    let filepath = Path::new(&filepath_);
    let filehash = match tools::filehash::get_hash(&filepath_) {
        Ok(h) => Some(h),
        Err(e) => {
            println!("Couldn't obtain hash of file: {} due to {}", filepath_, e);
            None
        }
    };

    let newpath = match filepath.file_name() {
        Some(f) => {
            let prefix = match filehash.as_ref() {
                Some(h) => {
                    let prefix_1 = Path::new(&h[0..1]);
                    let prefix_2 = Path::new(&h[0..2]);
                    repopath.join(prefix_1).join(prefix_2)
                },
                None => {
                    repopath.join("nohash/")
                }
            };
            match std::fs::create_dir_all(&prefix) {
                Ok(()) => {},
                Err(_) => {
                    println!("Cannot create directory.");
                }
            };
            let new = prefix.join(f);
            std::fs::copy(&filepath, &new).unwrap();
            new.to_str().unwrap().to_string()
        },
        None => filepath_.clone()
    };
    
    (newpath, filehash)
}

pub fn insert(connection: &SqliteConnection,
              title_: String,
              filepath_: String,
              datetime_utc: &DateTime<Utc>) {
    let datetime_ = datetime_utc.naive_utc();
    let slug_ = tools::text::slugify(&title_);
    let (newpath_, filehash_) = prepare_file(&filepath_);

    let item_ = models::NewRepoItem {
        datetime: datetime_,
        title: title_,
        slug: slug_,
        description: Some("".to_string()),
        category_id: 0,
        filepath: newpath_,
        filetype: Some("".to_string()),
        filehash: filehash_,
        published: false };

    diesel::insert_into(repo_items::table)
        .values(&item_)
        .execute(connection)
        .expect("Error inserting new item");
}

pub fn insert_full(connection: &SqliteConnection, item_: &models::NewRepoItem) {
    diesel::insert_into(repo_items::table)
        .values(item_)
        .execute(connection)
        .expect("Error inserting new item");
}

pub fn query(connection: &SqliteConnection) -> Vec<models::RepoItem> {
    repo_items::table
        .load::<models::RepoItem>(connection)
        .expect("Error loading repo_items")
}

pub fn query_published(connection: &SqliteConnection) -> Vec<models::RepoItem> {
    repo_items::table
        .filter(repo_items::published.eq(true))
        .order(repo_items::datetime.desc())
        .load::<models::RepoItem>(connection)
        .expect("Error loading repo_items")
}

pub fn query_published_by_category(connection: &SqliteConnection,
                                   slug: &str) -> Vec<models::RepoItem> {

    let cat_id: i32 = categories::table
        .select(categories::id)
        .filter(categories::slug.like(slug))
        .first::<i32>(connection).unwrap_or(0);

    repo_items::table
        .filter(repo_items::published.eq(true).and(
                repo_items::category_id.eq(cat_id))
            )
        .order(repo_items::datetime.desc())
        .load::<models::RepoItem>(connection)
        .expect("Error loading repo_items")
}

pub fn remove(connection: &SqliteConnection, id: i32) {
    diesel::delete(repo_items::table.filter(repo_items::id.eq(id)))
        .execute(connection)
        .expect(&format!("Error removing item with id = {}", id));
}

pub fn drop_all(connection: &SqliteConnection) {
    diesel::delete(repo_items::table)
        .execute(connection)
        .expect(&format!("Error removing all repo_items"));
}

pub fn query_newest(connection: &SqliteConnection, last: i64) -> Vec<models::RepoItem> {
    repo_items::table
        .order(repo_items::datetime.desc())
        .limit(last)
        .load::<models::RepoItem>(connection)
        .expect("Error loading repo_items")
}

pub fn get(connection: &SqliteConnection, id: i32) -> Result<models::RepoItem, diesel::result::Error> {
    repo_items::table
        .filter(repo_items::id.eq(id))
        .first::<models::RepoItem>(connection)
}

pub fn get_by_slug(connection: &SqliteConnection, slug: String) -> Result<models::RepoItem, diesel::result::Error> {
    repo_items::table
        .filter(repo_items::slug.eq(slug))
        .first::<models::RepoItem>(connection)
}

pub fn update(connection: &SqliteConnection, item: &models::RepoItem) {
    let (newpath, filehash) = prepare_file(&item.filepath);
    diesel::update(repo_items::table.filter(repo_items::id.eq(item.id)))
        .set((repo_items::title.eq(&item.title),
              repo_items::slug.eq(&item.slug),
              repo_items::datetime.eq(&item.datetime),
              repo_items::published.eq(&item.published),
              repo_items::description.eq(&item.description),
              repo_items::category_id.eq(&item.category_id),
              repo_items::filepath.eq(&newpath),
              repo_items::filehash.eq(&filehash),
              repo_items::filetype.eq(&item.filetype),
        ))
        .execute(connection)
        .expect(&format!("Error updating item with id = {}", item.id));
}
