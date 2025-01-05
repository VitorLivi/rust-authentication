use crate::{
    core::application::use_cases::create_user::CreateUserUseCase,
    core::application::use_cases::create_user::CreateUserUseCaseInputDto,
    core::application::use_cases::find_user::FindUserUseCase,
    core::application::use_cases::list_user::ListUserUseCase,
    core::application::use_cases::update_user::UpdateUserUseCase,
    core::application::use_cases::update_user::UpdateUserUseCaseInputDto,
    core::infra::db::diesel::user::user_repository::UserDieselRepository, schema::user,
    shared::application::use_cases::use_case::UseCase, webserver::config::database::Database,
};
use actix_session::Session;
use actix_web::{get, post, put, web, HttpResponse, Responder};

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

#[put("/update")]
pub async fn update(
    session: Session,
    payload: web::Json<UpdateUserUseCaseInputDto>,
) -> impl Responder {
    let pool = Database::get_pool();
    let user_repository = UserDieselRepository::new(pool.get().unwrap(), user::table);
    let mut update_user = UpdateUserUseCase::new(session, Box::new(user_repository));

    let result = update_user.execute(payload.0);

    match result {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(message) => {
            if message == "User not found" {
                HttpResponse::NotFound().body(message)
            } else {
                HttpResponse::BadRequest().body(message)
            }
        }
    }
}

#[get("/list")]
pub async fn list(session: Session) -> impl Responder {
    let pool = Database::get_pool();
    let user_repository = UserDieselRepository::new(pool.get().unwrap(), user::table);
    let mut list_users = ListUserUseCase::new(session, Box::new(user_repository));

    let users = list_users.execute(());

    HttpResponse::Ok().json(users)
}

#[get("/find-user/{id}")]
pub async fn find(session: Session, path: web::Path<String>) -> impl Responder {
    let pool = Database::get_pool();
    let user_repository = UserDieselRepository::new(pool.get().unwrap(), user::table);
    let mut find_user = FindUserUseCase::new(session, Box::new(user_repository));

    let id = path.into_inner();

    if id.is_empty() {
        return HttpResponse::BadRequest().body("Id is required");
    }

    let user = find_user.execute(id);

    match user {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(message) => HttpResponse::NotFound().body(message),
    }
}

pub fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(sign_up);
    cfg.service(update);
    cfg.service(list);
    cfg.service(find);
}
