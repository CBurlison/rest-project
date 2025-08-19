#[macro_use]
extern crate html5ever;
extern crate markup5ever_rcdom as rcdom;

use actix_web::{self, main, web, App, HttpResponse, HttpServer};
mod routes;
mod controllers;
mod helpers;
mod html_modal;

#[main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .configure(routes::auth::add_routes)
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