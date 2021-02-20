#[macro_use]
extern crate diesel;

pub mod models;
pub mod schema;

use diesel::r2d2::ConnectionManager;
use diesel::{MysqlConnection, r2d2};
use dotenv::dotenv;
use std::env;

pub type DbPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

#[derive(Clone)]
pub struct DbConnection {
    pub pool: DbPool,
}

impl DbConnection {
    pub fn new() -> Self {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<MysqlConnection>::new(database_url);
        let pool = r2d2::Pool::builder().build(manager).expect("Failed to create pool.");

        DbConnection {
            pool
        }
    }
}
