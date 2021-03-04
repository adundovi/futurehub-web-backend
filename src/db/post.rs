use diesel::{prelude::*, sqlite::SqliteConnection};
use chrono::prelude::*;

use crate::db::models;
use crate::db::sqlite_schema::posts as posts;
use crate::tools;

pub fn insert(connection: &SqliteConnection, title_: String, datetime_utc: &DateTime<Utc>) {
    let datetime_ = datetime_utc.naive_utc();
    let other_slug = tools::text::slugify(&title_);
    let post_ = models::NewPost {
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

pub fn insert_full(connection: &SqliteConnection, post_: &models::NewPost) {
    diesel::insert_into(posts::table)
        .values(post_)
        .execute(connection)
        .expect("Error inserting new post");
}

pub fn query(connection: &SqliteConnection) -> Vec<models::Post> {
    posts::table
        .load::<models::Post>(connection)
        .expect("Error loading posts")
}

pub fn query_published(connection: &SqliteConnection) -> Vec<models::Post> {
    posts::table
        .filter(posts::published.eq(true))
        .order(posts::datetime.desc())
        .load::<models::Post>(connection)
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

pub fn query_newest(connection: &SqliteConnection, last: i64) -> Vec<models::Post> {
    posts::table
        .order(posts::datetime.desc())
        .limit(last)
        .load::<models::Post>(connection)
        .expect("Error loading posts")
}

pub fn get(connection: &SqliteConnection, id: i32) -> Result<models::Post, diesel::result::Error> {
    posts::table
        .filter(posts::id.eq(id))
        .first::<models::Post>(connection)
}

pub fn update(connection: &SqliteConnection, post: &models::Post) {
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
