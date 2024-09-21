use actix_session::Session;
use actix_web::{
    get, post,
    web::{self, ServiceConfig},
    App, HttpResponse, HttpServer, Responder,
};

use crate::{
    core::{
        application::use_cases::authenticate_user::{AuthenticateUser, AuthenticateUserInputDto},
        domain::entities::authenticator::Authenticator,
    },
    shared::use_cases::use_case::UseCase,
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
