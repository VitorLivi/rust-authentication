use actix_session::Session;
use actix_web::{get, post, web::ServiceConfig, HttpResponse, Responder};

use crate::shared::application::use_cases::use_case::UseCase;

use crate::core::application::use_cases::authenticate_user::{
    AuthenticateUser, AuthenticateUserInputDto,
};

#[post("/authenticate")]
pub async fn authenticate(session: Session) -> impl Responder {
    let input = AuthenticateUserInputDto::new("username".to_string(), "123".to_string(), session);

    let authenticate_user = AuthenticateUser::new();
    authenticate_user.execute(input);

    HttpResponse::Ok().body("OK")
}

pub fn config_routes(cfg: &mut ServiceConfig) {
    cfg.service(authenticate);
}
