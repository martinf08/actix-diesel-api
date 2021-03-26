pub use crate::schema::roles;
pub use crate::schema::users;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Queryable, Debug)]
pub struct User {
    pub id: i32,
    pub name: Option<String>,
    pub password: Option<String>,
    pub role_id: i32,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a, 'b, 'c> {
    pub name: &'a str,
    pub password: &'b str,
    pub role_id: &'c i32,
}

#[derive(Insertable, Clone, Deserialize, Queryable, Debug)]
#[table_name = "users"]
pub struct PartialUser {
    pub name: String,
    pub password: String,
    pub role_id: i32,
}

impl From<User> for PartialUser {
    fn from(user: User) -> Self {
        PartialUser {
            name: user.name.unwrap(),
            password: user.password.unwrap(),
            role_id: user.role_id,
        }
    }
}

#[derive(Queryable, Deserialize)]
pub struct SlimUser {
    pub name: String,
    pub password: String,
}

impl From<User> for SlimUser {
    fn from(user: User) -> Self {
        SlimUser {
            name: user.name.unwrap(),
            password: user.password.unwrap(),
        }
    }
}

#[derive(Insertable, Serialize, Deserialize, Queryable, Debug)]
#[table_name = "roles"]
pub struct Role {
    pub id: i32,
    pub name: String,
}
