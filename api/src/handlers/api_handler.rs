use actix_web::{web, HttpRequest, HttpResponse, Responder};
use db::DbPool;
use diesel::prelude::*;
use diesel::{ExpressionMethods, QueryDsl};
use serde::Deserialize;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/{id}").route(web::get().to(index)));
    cfg.service(web::resource("/").route(web::get().to(list)));
}

async fn index(pool: web::Data<DbPool>, user_id: web::Path<String>) -> impl Responder {
    use db::schema::users::dsl::*;

    let user = users
        .filter(id.eq(user_id.parse::<i32>().unwrap()))
        .load::<db::models::User>(&*pool.get().unwrap())
        .expect("Error loading");

    return HttpResponse::Ok().json(user);
}

#[derive(Deserialize, Debug)]
struct QueryExtracted {
    list: Option<String>,
}

async fn list(
    pool: web::Data<DbPool>,
    web::Query(QueryExtracted { list }): web::Query<QueryExtracted>,
) -> impl Responder {
    use db::schema::users::dsl::*;

    match list {
        Some(s) => {
            let n = match s.parse::<i64>() {
                Ok(n) => n,
                Err(_e) => return HttpResponse::BadRequest().finish(),
            };

            let result = users
                .limit(n)
                .load::<db::models::User>(&*pool.get().unwrap())
                .expect("Error loading");

            return HttpResponse::Ok().json(result);
        }
        None => HttpResponse::BadRequest().finish(),
    }
}

/// 404 handler
pub async fn not_found(req: HttpRequest) -> impl Responder {
    return format!(r#"Cannot GET {}"#, req.path());
}
