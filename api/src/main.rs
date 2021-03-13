extern crate env_logger;

mod handlers;

use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{App, HttpResponse, HttpServer, web, guard};
use crate::handlers::not_found;
use setup;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    match env::args()
        .skip(1)
        .collect::<Vec<String>>()
        .pop() {
        Some(s) => if s == "setup" { setup::init() },
        None => ()
    }

    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let pool = db::DbConnection::new();
    HttpServer::new(
        move || App::new()
            .wrap(Logger::default())
            .wrap(Cors::default().allow_any_origin())
            .data(pool.pool.clone())
            .service(web::scope("/api").configure(handlers::api_config))
            .default_service(
                // 404 for GET request
                web::resource("")
                    .route(web::get().to(|req| not_found(req)))
                // all requests that are not `GET`
                .route(
                    web::route()
                        .guard(guard::Not(guard::Get()))
                        .to(HttpResponse::MethodNotAllowed),
                ),
            )
    )
    .bind("0.0.0.0:5001")?
    .run()
    .await
}
