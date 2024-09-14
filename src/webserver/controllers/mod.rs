use actix_web::web;

pub mod authentication_controller;

pub fn config_all_routes(cfg: &mut web::ServiceConfig) {
    authentication_controller::config_routes(cfg)
}
