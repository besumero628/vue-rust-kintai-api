use crate::controllers::{users_controller, works_controller};
use actix_web::web;

pub fn user(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/user")
            .route(web::get().to(users_controller::show))
            .route(web::post().to(users_controller::create))
    )
    .service(
        web::resource("/user/{user_id}")
            .route(web::put().to(users_controller::update))
    );
}

pub fn work(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/work")
            .route(web::get().to(works_controller::index))
            .route(web::post().to(works_controller::create))
            .route(web::put().to(works_controller::update))
            .route(web::delete().to(works_controller::delete)),
    );
}