pub use crate::schema::users;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Queryable, Debug)]
pub struct User {
    pub id: i32,
    pub name: Option<String>,
    pub password: Option<String>,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a, 'b> {
    pub name: &'a str,
    pub password: &'b str,
}

#[derive(Clone, Deserialize)]
pub struct PartialUser {
    pub name: String,
    pub password: String,
}