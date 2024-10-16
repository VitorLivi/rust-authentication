use crate::{
    core::{
        application::use_cases::create_user::{CreateUser, CreateUserInputDto},
        infra::db::diesel::user::user_repository::UserDieselRepository,
    },
    schema::user,
    shared::application::use_cases::use_case::UseCase,
    webserver::config::database::Database,
};
use actix_web::{post, web, HttpResponse, Responder};

#[post("/sign_up")]
pub async fn sign_up(payload: web::Json<CreateUserInputDto>) -> impl Responder {
    let pool = Database::get_pool();
    let user_repository = UserDieselRepository::new(pool.get().unwrap(), user::table);
    let create_user = CreateUser::new(Box::new(user_repository));

    create_user.execute(payload.0);

    HttpResponse::Ok().body("OK")
}

pub fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(sign_up);
}
