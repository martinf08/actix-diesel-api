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

#[derive(Serialize)]
pub struct SlimUser {
    pub name: String,
}

impl From<User> for SlimUser {
    fn from(user: User) -> Self {
        SlimUser {
            name: user.name.unwrap()
        }
    }
}