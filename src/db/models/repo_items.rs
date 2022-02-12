use diesel::{prelude::*, sqlite::SqliteConnection};
use chrono::prelude::*;
use std::path::Path;

use crate::db::sqlite_schema::repo_items as repo_items;
use crate::db::sqlite_schema::categories as categories;
use crate::tools::{filehash, import, text};

#[derive(Debug, Insertable, Serialize, Deserialize, Eq, Ord, PartialEq, PartialOrd)]
#[table_name = "repo_items"]
#[serde(rename_all = "PascalCase")]
pub struct NewRepoItem {
    #[serde(with= "import::date_serializer")]
    pub datetime: NaiveDateTime,
    pub title: String,
    pub slug: String,
    pub filepath: String,
    pub description: Option<String>,
    pub category_id: i32,
    pub filetype: Option<String>,
    pub filehash: Option<String>,
    pub filesize: Option<i64>,
    pub published: bool,
}

#[derive(Queryable, Clone)]
pub struct RepoItem {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub filepath: String,
    pub description: Option<String>,
    pub category_id: i32,
    pub filetype: Option<String>,
    pub published: bool,
    pub datetime: NaiveDateTime, // UTC
    pub filehash: Option<String>,
    pub filesize: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
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


fn get_repo_path() -> std::path::PathBuf {
    let c = crate::active_config();
    let c_repo = c.get_extra("repository").unwrap();
    let p = Path::new(
            c_repo["path"].as_str().unwrap()
    );
    p.to_path_buf()
}

fn get_hash(filepath: &String) -> Option<String> {
    match filehash::get_hash(&filepath) {
        Ok(h) => Some(h),
        Err(e) => {
            println!("Couldn't obtain hash of file: {} due to {}", filepath, e);
            None
        }
    }
}

fn get_filesize(filepath: &String) -> Option<i64> {
    let metadata = std::fs::metadata(filepath);
    match metadata {
        Ok(m) => Some(m.len() as i64),
        Err(_) => None,
    }
}

fn prepare_file(filepath_: &String) -> (String, Option<String>, Option<i64>) {

    let filehash = get_hash(&filepath_);
    let filesize = get_filesize(&filepath_);
    let filepath = Path::new(&filepath_);
    let repopath = get_repo_path();
    
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
            if !new.exists() {
                std::fs::copy(&filepath, &new).unwrap();
            }
            new.to_str().unwrap().to_string()
        },
        None => filepath_.clone()
    };
    
    (newpath, filehash, filesize)
}

pub fn insert(connection: &SqliteConnection,
              title_: String,
              filepath_: String,
              datetime_utc: &DateTime<Utc>,
              cat_: i32,
              publish_: bool) {
    let datetime_ = datetime_utc.naive_utc();
    let slug_ = text::slugify(&title_);
    let (newpath_, filehash_, filesize_) = prepare_file(&filepath_);

    let item_ = NewRepoItem {
        datetime: datetime_,
        title: title_,
        slug: slug_,
        description: Some("".to_string()),
        category_id: cat_,
        filepath: newpath_,
        filetype: Some("".to_string()),
        filehash: filehash_,
        filesize: filesize_, 
        published: publish_ };

    diesel::insert_into(repo_items::table)
        .values(&item_)
        .execute(connection)
        .expect("Error inserting new item");
}

pub fn insert_full(connection: &SqliteConnection, item_: &NewRepoItem) {
    diesel::insert_into(repo_items::table)
        .values(item_)
        .execute(connection)
        .expect("Error inserting new item");
}

pub fn query(connection: &SqliteConnection) -> Vec<RepoItem> {
    repo_items::table
        .load::<RepoItem>(connection)
        .expect("Error loading repo_items")
}

pub fn query_published(connection: &SqliteConnection) -> Vec<RepoItem> {
    repo_items::table
        .filter(repo_items::published.eq(true))
        .order(repo_items::datetime.desc())
        .load::<RepoItem>(connection)
        .expect("Error loading repo_items")
}

pub fn query_published_by_category(connection: &SqliteConnection,
                                   slug: &str) -> Vec<RepoItem> {

    let cat_id: i32 = categories::table
        .select(categories::id)
        .filter(categories::slug.like(slug))
        .first::<i32>(connection).unwrap_or(0);

    repo_items::table
        .filter(repo_items::published.eq(true).and(
                repo_items::category_id.eq(cat_id))
            )
        .order(repo_items::datetime.desc())
        .load::<RepoItem>(connection)
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

pub fn query_newest(connection: &SqliteConnection, last: i64) -> Vec<RepoItem> {
    repo_items::table
        .order(repo_items::datetime.desc())
        .limit(last)
        .load::<RepoItem>(connection)
        .expect("Error loading repo_items")
}

pub fn get(connection: &SqliteConnection, id: i32) -> Result<RepoItem, diesel::result::Error> {
    repo_items::table
        .filter(repo_items::id.eq(id))
        .first::<RepoItem>(connection)
}

pub fn get_by_slug(connection: &SqliteConnection, slug: String) -> Result<RepoItem, diesel::result::Error> {
    repo_items::table
        .filter(repo_items::slug.eq(slug))
        .first::<RepoItem>(connection)
}

pub fn update(connection: &SqliteConnection, item: &RepoItem) {
    let (newpath, filehash, filesize) = prepare_file(&item.filepath);
    diesel::update(repo_items::table.filter(repo_items::id.eq(item.id)))
        .set((repo_items::title.eq(&item.title),
              repo_items::slug.eq(&item.slug),
              repo_items::datetime.eq(&item.datetime),
              repo_items::published.eq(&item.published),
              repo_items::description.eq(&item.description),
              repo_items::category_id.eq(&item.category_id),
              repo_items::filepath.eq(&newpath),
              repo_items::filehash.eq(&filehash),
              repo_items::filesize.eq(&filesize),
              repo_items::filetype.eq(&item.filetype),
        ))
        .execute(connection)
        .expect(&format!("Error updating item with id = {}", item.id));
}
