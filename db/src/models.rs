pub use crate::schema::users;
use serde::Serialize;

#[derive(Serialize, Queryable, Debug)]
pub struct User {
    pub id: i32,
    pub name: Option<String>,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub name: &'a str,
}
