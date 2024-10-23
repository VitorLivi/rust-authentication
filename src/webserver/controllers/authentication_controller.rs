use crate::core::infra::db::diesel::user::user_repository::UserDieselRepository;
use crate::shared::application::use_cases::use_case::UseCase;
use actix_session::Session;
use actix_web::web::Json;
use actix_web::{post, web::ServiceConfig, HttpResponse, Responder};

use crate::core::application::use_cases::authenticate_user::{
    AuthenticateUserUseCase, AuthenticateUserUseCaseInputDto,
};
use crate::schema::user;
use crate::webserver::config::database::Database;

#[post("/authenticate")]
pub async fn authenticate(
    session: Session,
    body: Json<AuthenticateUserUseCaseInputDto>,
) -> impl Responder {
    let pool = Database::get_pool();
    let user_repository = UserDieselRepository::new(pool.get().unwrap(), user::table);
    let mut authenticate_user = AuthenticateUserUseCase::new(session, Box::new(user_repository));

    authenticate_user.execute(body.0);

    HttpResponse::Ok().body("OK")
}

pub fn config_routes(cfg: &mut ServiceConfig) {
    cfg.service(authenticate);
}
