use actix_identity::Identity;
use actix_identity;
use actix_web::{web, HttpResponse};
use crate::errors::ServiceError;
use db::DbPool;
use db::models::{User, SlimUser};
use diesel::prelude::*;
use diesel::{QueryDsl, ExpressionMethods};
use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct PartialUser {
    pub name: String,
    password: String,
}

pub async fn login(
    partial_user: web::Json<PartialUser>,
    id: Identity,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, ServiceError> {
    match web::block(move || query_user(partial_user.into_inner(), pool)).await {
        Ok(users) => {
            let (user_data, user_found) = users;
            if user_data.password != user_found.password {
                return Err(ServiceError::InternalServerError);
            }
            id.remember(user_data.name.to_owned());
            Ok(HttpResponse::Ok().finish())
        }
        Err(_) => Err(ServiceError::InternalServerError)
    }
}

pub async fn is_logged(id: Identity) -> HttpResponse {
    if let Some(id) = id.identity() {
        HttpResponse::Ok().json(format!("Hello {}", id))
    } else {
        HttpResponse::Ok().json("Welcome Anonymous")
    }

}

pub async fn logout(id: Identity) -> HttpResponse {
    id.forget();
    HttpResponse::Ok().finish()
}

fn query_user(partial_user: PartialUser, pool: web::Data<DbPool>) -> Result<(PartialUser, PartialUser), ServiceError> {
    use db::schema::users::dsl::*;
    let conn = &pool.get().unwrap();
    let mut users_found = users.
        filter(name.eq(&partial_user.name))
        .load::<User>(conn)?;

    let user_found = users_found.pop().unwrap();
    let user_found: PartialUser = PartialUser { name: user_found.name.unwrap(), password: user_found.password.unwrap() };

    Ok((partial_user.clone(), user_found))
}