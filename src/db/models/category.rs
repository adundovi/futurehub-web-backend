use diesel::{prelude::*, sqlite::SqliteConnection};

use crate::db::sqlite_schema::categories as categories;
use crate::tools;

#[derive(Debug, Insertable, Serialize, Deserialize, Eq, Ord, PartialEq, PartialOrd)]
#[table_name = "categories"]
#[serde(rename_all = "PascalCase")]
pub struct NewCategory {
    pub title: String,
    pub slug: String,
    pub icon: Option<String>,
    pub description: Option<String>,
}

#[derive(Queryable, Clone)]
pub struct Category {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub icon: Option<String>,
    pub description: Option<String>,
}


pub fn insert(connection: &SqliteConnection,
              title_: String) {
    let other_slug = tools::text::slugify(&title_);
    let category_ = NewCategory {
        title: title_,
        slug: other_slug,
        description: None,
        icon: None };

    diesel::insert_into(categories::table)
        .values(&category_)
        .execute(connection)
        .expect("Error inserting new category");
}

pub fn insert_full(connection: &SqliteConnection, category_: &NewCategory) {
    diesel::insert_into(categories::table)
        .values(category_)
        .execute(connection)
        .expect("Error inserting new category");
}

pub fn query(connection: &SqliteConnection) -> Vec<Category> {
    categories::table
        .load::<Category>(connection)
        .expect("Error loading categories")
}

pub fn remove(connection: &SqliteConnection, id: i32) {
    diesel::delete(categories::table.filter(categories::id.eq(id)))
        .execute(connection)
        .expect(&format!("Error removing category with id = {}", id));
}

pub fn drop_all(connection: &SqliteConnection) {
    diesel::delete(categories::table)
        .execute(connection)
        .expect(&format!("Error removing all categories"));
}

pub fn get(connection: &SqliteConnection, id: i32) -> Result<Category, diesel::result::Error> {
    categories::table
        .filter(categories::id.eq(id))
        .first::<Category>(connection)
}

pub fn get_by_slug(connection: &SqliteConnection, slug: String) -> Result<Category, diesel::result::Error> {
    categories::table
        .filter(categories::slug.eq(slug))
        .first::<Category>(connection)
}

pub fn update(connection: &SqliteConnection, category: &Category) {
    diesel::update(categories::table.filter(categories::id.eq(category.id)))
        .set((categories::title.eq(&category.title),
              categories::slug.eq(&category.slug),
              categories::icon.eq(&category.icon),
              categories::description.eq(&category.description),
        ))
        .execute(connection)
        .expect(&format!("Error updating category with id = {}", category.id));
}
