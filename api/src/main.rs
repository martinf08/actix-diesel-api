extern crate env_logger;

mod errors;
mod handlers;

use actix_cors::Cors;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::middleware::Logger;
use actix_web::{guard, web, App, HttpResponse, HttpServer};
use setup;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    match env::args().skip(1).collect::<Vec<String>>().pop() {
        Some(s) => {
            if s == "setup" {
                setup::init()
            }
        }
        None => (),
    }

    std::env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    let pool = db::DbConnection::new();
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Cors::permissive())
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&[0; 32])
                    .name("auth-cookie")
                    .secure(false),
            ))
            .data(pool.pool.clone())
            .service(web::scope("/api").configure(handlers::api_handler::config))
            .service(web::scope("/auth").configure(handlers::auth_handler::config))
            .service(web::scope("/register").configure(handlers::register_handler::config))
            .default_service(
                // 404 for GET request
                web::resource("")
                    .route(web::get().to(|req| handlers::api_handler::not_found(req)))
                    // all requests that are not `GET`
                    .route(
                        web::route()
                            .guard(guard::Not(guard::Get()))
                            .to(HttpResponse::MethodNotAllowed),
                    ),
            )
    })
    .bind("0.0.0.0:5001")?
    .run()
    .await
}
