use crate::errors::ServiceError;
use actix_identity;
use actix_identity::Identity;
use actix_web::web::Json;
use actix_web::{web, HttpResponse};
use bcrypt::verify;
use db::models::{SlimUser, User};
use db::DbPool;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::{ExpressionMethods, QueryDsl};
use std::sync::{Arc, Mutex};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/login").route(web::post().to(login)))
        .service(web::resource("/is_logged").to(is_logged))
        .service(web::resource("/logout").to(logout));
}

pub async fn login(
    slim_user: web::Json<SlimUser>,
    id: Identity,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, ServiceError> {
    let user_cloned = Arc::new(Mutex::new(slim_user));
    let user_closure = user_cloned.clone();
    match web::block(move || query_user(user_cloned, pool)).await {
        Ok(user_found) => {
            if !verify(
                &user_closure.clone().lock().unwrap().password,
                &*user_found.password,
            )
            .unwrap()
            {
                return Err(ServiceError::InternalServerError);
            }
            id.remember(user_found.name.to_owned());
            Ok(HttpResponse::Ok().finish())
        }
        Err(_) => Err(ServiceError::InternalServerError),
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

fn query_user(
    slim_user: Arc<Mutex<Json<SlimUser>>>,
    pool: web::Data<DbPool>,
) -> Result<SlimUser, Error> {
    use db::schema::users::dsl::{name as user_name, users};

    let conn = &pool.get().unwrap();

    let user: QueryResult<User> = users
        .filter(user_name.eq(&slim_user.clone().lock().unwrap().name))
        .first(conn);

    Ok(SlimUser::from(user.unwrap()))
}
