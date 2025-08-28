use actix_web::{self, main, web, App, HttpResponse, HttpServer};
use async_std::task;
mod config;
mod controllers;
mod helpers;
mod html_modal;

#[main]
async fn main() -> std::io::Result<()> {
    task::block_on(config::db::config_db());

    HttpServer::new(|| {
        App::new()
        .configure(config::auth::add_routes)
        .route("/", web::get().to(route_default))
        .default_service(web::route().to(default_svc))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

async fn route_default() -> HttpResponse {
    HttpResponse::Ok().body("/")
}

async fn default_svc() -> HttpResponse {
    HttpResponse::Ok().body("There is nothing here!")
}
