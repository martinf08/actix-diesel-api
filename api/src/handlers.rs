use actix_web::{web, Responder, HttpResponse, HttpRequest};
use db::DbPool;
use diesel::prelude::*;
use diesel::{QueryDsl, ExpressionMethods};

pub fn api_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/{id}")
            .route(web::get().to(index))
    );
}

async fn index(pool: web::Data<DbPool>, user_id: web::Path<String>) -> impl Responder {
    use db::schema::users::dsl::*;

    let user = users
        .filter(id.eq(user_id.parse::<i32>().unwrap()))
        .load::<db::models::User>(&*pool.get().unwrap())
        .expect("Error loading posts");

    return HttpResponse::Ok().json(user)
}

/// 404 handler
pub async fn not_found(req: HttpRequest) -> impl Responder {
    return format!(r#"Cannot GET {}"#, req.path());
}