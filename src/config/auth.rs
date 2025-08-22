use actix_web::{web};
use super::super::controllers::auth;

pub fn add_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("", web::get().to(auth::hello))
            .route("/echo", web::patch().to(auth::echo))
            .route("/hey", web::post().to(auth::manual_hello))
    );
}