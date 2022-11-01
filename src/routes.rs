use crate::controllers::test_controller;
use actix_web::{web, HttpResponse};

pub fn scoped_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/test")
            .route(web::get().to(|| async { HttpResponse::Ok().body("test!") }))
            .route(web::head().to(HttpResponse::MethodNotAllowed)),
    );
}

// this function could be located in a different module
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/app")
            .route(web::get().to(|| async { HttpResponse::Ok().body("app!") }))
            .route(web::head().to(HttpResponse::MethodNotAllowed)),
    );
}

pub fn test(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/test")
            .route(web::get().to(test_controller::index))
            .route(web::get().to(test_controller::new))
            .route(web::post().to(test_controller::create))
    )
    .service(
        web::resource("/test/{user_id}")
            .route(web::get().to(test_controller::show))
            .route(web::put().to(test_controller::update))
            .route(web::delete().to(test_controller::delete))
    )
    .service(
        web::resource("/test/{user_id}/edit")
        .route(web::get().to(test_controller::edit))
    );
}