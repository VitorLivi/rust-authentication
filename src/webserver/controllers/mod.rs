use actix_web::web;

pub mod authentication_controller;
pub mod user_management_controller;

pub fn config_all_routes(cfg: &mut web::ServiceConfig) {
    authentication_controller::config_routes(cfg);
    user_management_controller::config_routes(cfg);
}
