use diesel::{prelude::*, sqlite::SqliteConnection};
use chrono::prelude::*;

use crate::db::sqlite_schema::posts as posts;
use crate::tools::{import, text};

#[derive(Debug, Insertable, Serialize, Deserialize, Eq, Ord, PartialEq, PartialOrd)]
#[table_name = "posts"]
#[serde(rename_all = "PascalCase")]
pub struct NewPost {
    #[serde(with= "import::date_serializer")]
    pub datetime: NaiveDateTime,
    pub title: String,
    pub slug: String,
    pub body: Option<String>,
    pub published: bool,
}

#[derive(Queryable, Clone)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub body: Option<String>,
    pub published: bool,
    pub datetime: NaiveDateTime, // UTC
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostAttribs {
    pub title: String,
    pub slug: String,
    pub datetime: NaiveDateTime,
    pub body: Option<String>,
}


pub fn insert(connection: &SqliteConnection, title_: String, datetime_utc: &DateTime<Utc>) {
    let datetime_ = datetime_utc.naive_utc();
    let other_slug = text::slugify(&title_);
    let post_ = NewPost {
        datetime: datetime_,
        title: title_,
        slug: other_slug,
        body: Some("".to_string()),
        published: false };

    diesel::insert_into(posts::table)
        .values(&post_)
        .execute(connection)
        .expect("Error inserting new post");
}

pub fn insert_full(conn: &SqliteConnection, post_: &NewPost) {
    diesel::insert_into(posts::table)
        .values(post_)
        .execute(conn)
        .expect("Error inserting new post");
}

pub fn get_all(conn: &SqliteConnection) -> Vec<Post> {
    posts::table
        .load::<Post>(conn)
        .expect("Error loading posts")
}

pub fn get_all_published(conn: &SqliteConnection) -> Vec<Post> {
    posts::table
        .filter(posts::published.eq(true))
        .order(posts::datetime.desc())
        .load::<Post>(conn)
        .expect("Error loading posts")
}

pub fn remove(connection: &SqliteConnection, id: i32) {
    diesel::delete(posts::table.filter(posts::id.eq(id)))
        .execute(connection)
        .expect(&format!("Error removing post with id = {}", id));
}

pub fn drop_all(connection: &SqliteConnection) {
    diesel::delete(posts::table)
        .execute(connection)
        .expect(&format!("Error removing all posts"));
}

pub fn query_newest(connection: &SqliteConnection, last: i64) -> Vec<Post> {
    posts::table
        .order(posts::datetime.desc())
        .limit(last)
        .load::<Post>(connection)
        .expect("Error loading posts")
}

pub fn get(connection: &SqliteConnection, id: i32) -> Result<Post, diesel::result::Error> {
    posts::table
        .filter(posts::id.eq(id))
        .first::<Post>(connection)
}

pub fn get_by_slug(connection: &SqliteConnection, slug: String) -> Result<Post, diesel::result::Error> {
    posts::table
        .filter(posts::slug.eq(slug))
        .first::<Post>(connection)
}

pub fn update(connection: &SqliteConnection, post: &Post) {
    diesel::update(posts::table.filter(posts::id.eq(post.id)))
        .set((posts::title.eq(&post.title),
              posts::slug.eq(&post.slug),
              posts::datetime.eq(&post.datetime),
              posts::published.eq(&post.published),
              posts::body.eq(&post.body),
        ))
        .execute(connection)
        .expect(&format!("Error updating post with id = {}", post.id));
}
