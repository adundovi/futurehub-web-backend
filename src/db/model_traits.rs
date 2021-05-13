use diesel::sqlite::SqliteConnection;
use diesel::result::QueryResult;

pub trait Queries {
    fn get_all(conn: &SqliteConnection) -> QueryResult<Vec<Self>> 
        where Self: std::marker::Sized;
    fn get(conn: &SqliteConnection, id: i32) -> QueryResult<Self> 
        where Self: std::marker::Sized;
    fn drop_all(conn: &SqliteConnection) -> QueryResult<usize>;
    fn remove(conn: &SqliteConnection, id: i32) -> QueryResult<usize>;
}
