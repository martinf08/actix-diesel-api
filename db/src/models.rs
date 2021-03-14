pub use crate::schema::users;
use serde::Serialize;

#[derive(Serialize, Queryable, Debug)]
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

pub struct SlimUser {
    pub name: Option<String>,
}

impl From<User> for SlimUser {
    fn from(user: User) -> Self {
        SlimUser {
            name: user.name
        }
    }
}