use crate::{
    core::application::use_cases::create_user::CreateUserUseCase,
    core::application::use_cases::create_user::CreateUserUseCaseInputDto,
    core::application::use_cases::list_user::ListUserUseCase,
    core::infra::db::diesel::user::user_repository::UserDieselRepository, schema::user,
    shared::application::use_cases::use_case::UseCase, webserver::config::database::Database,
};
use actix_session::Session;
use actix_web::{get, post, web, HttpResponse, Responder};

#[post("/create")]
pub async fn sign_up(
    session: Session,
    payload: web::Json<CreateUserUseCaseInputDto>,
) -> impl Responder {
    let pool = Database::get_pool();
    let user_repository = UserDieselRepository::new(pool.get().unwrap(), user::table);
    let mut create_user = CreateUserUseCase::new(session, Box::new(user_repository));

    create_user.execute(payload.0);
    HttpResponse::Ok().body("OK")
}

#[get("/list")]
pub async fn list(session: Session) -> impl Responder {
    let pool = Database::get_pool();
    let user_repository = UserDieselRepository::new(pool.get().unwrap(), user::table);
    let mut list_users = ListUserUseCase::new(session, Box::new(user_repository));

    let users = list_users.execute(());

    HttpResponse::Ok().json(users)
}

pub fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(sign_up);
    cfg.service(list);
}
