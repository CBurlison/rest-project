use actix_web::{web};
use super::super::controllers::auth_controller;

pub fn add_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("", web::get().to(auth_controller::hello))
            .route("/echo", web::patch().to(auth_controller::echo))
            .route("/hey", web::post().to(auth_controller::manual_hello))
    );
}