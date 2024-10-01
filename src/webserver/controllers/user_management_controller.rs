use actix_web::{post, web::ServiceConfig, HttpResponse, Responder};

use crate::{
    core::application::use_cases::create_user::{CreateUser, CreateUserInputDto},
    shared::use_cases::use_case::UseCase,
};

#[post("/sign_up")]
pub async fn sign_up() -> impl Responder {
    let input = CreateUserInputDto::new("potato".to_string(), "123".to_string());

    let create_user = CreateUser::new();
    create_user.execute(input);

    HttpResponse::Ok().body("OK")
}

pub fn config_routes(cfg: &mut ServiceConfig) {
    cfg.service(sign_up);
}
