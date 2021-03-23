pub use crate::schema::users;
pub use crate::schema::roles;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Queryable, Debug)]
pub struct User {
    pub id: i32,
    pub name: Option<String>,
    pub password: Option<String>,
    pub role_id: Option<i32>
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a, 'b, 'c> {
    pub name: &'a str,
    pub password: &'b str,
    pub role_id: &'c i32,
}

#[derive(Clone, Deserialize)]
pub struct PartialUser {
    pub name: String,
    pub password: String,
}

#[derive(Insertable, Serialize, Deserialize, Queryable, Debug)]
#[table_name = "roles"]
pub struct Role {
    pub id: i32,
    pub name: String,
}