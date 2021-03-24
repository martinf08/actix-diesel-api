use actix_identity::Identity;
use actix_identity;
use actix_web::{web, HttpResponse};
use crate::errors::ServiceError;
use db::DbPool;
use db::models::{User, PartialUser};
use diesel::prelude::*;
use diesel::{QueryDsl, ExpressionMethods};
use bcrypt::verify;
use diesel::result::Error;
use std::sync::{Arc, Mutex};
use actix_web::web::Json;

pub async fn login(
    partial_user: web::Json<PartialUser>,
    id: Identity,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, ServiceError> {
    let user_cloned = Arc::new(Mutex::new(partial_user));
    let user_closure = user_cloned.clone();
    match web::block(move || query_user(user_cloned, pool)).await {
        Ok(user_found) => {
            if !verify(&user_closure.clone().lock().unwrap().password, &*user_found.password).unwrap() {
                return Err(ServiceError::InternalServerError);
            }
            id.remember(user_found.name.to_owned());
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

fn query_user(partial_user: Arc<Mutex<Json<PartialUser>>>, pool: web::Data<DbPool>) -> Result<PartialUser, Error> {
    use db::schema::users::dsl::{users, name as user_name};

    let conn = &pool.get().unwrap();

    let user: User = users
        .filter(user_name.eq(&partial_user.clone().lock().unwrap().name))
        .first(conn)
        .unwrap();

    Ok(PartialUser::from(user))
}