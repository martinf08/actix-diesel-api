use diesel::prelude::*;
use actix_identity;
use actix_web::{web, HttpResponse};
use db::models::{User, SlimUser};
use diesel::{QueryDsl, ExpressionMethods};
use db::{DbPool, DbConnection};
use futures::Future;
use diesel::result::Error;

fn query_user(auth_data: User, pool: web::Data<DbPool>) -> Vec<User> {
    use db::schema::users::dsl::*;
    let conn = &pool.get().unwrap();
    let mut users_found = users.
        filter(name.eq(&auth_data.name))
        .load::<User>(conn)
        .expect("Error");

    let mut result = Vec::new();
    result.push(auth_data);
    result.push(users_found.pop().unwrap());

    result
}