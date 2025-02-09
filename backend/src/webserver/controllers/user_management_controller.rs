use crate::{
    core::application::use_cases::create_user::CreateUserUseCase,
    core::application::use_cases::create_user::CreateUserUseCaseInputDto,
    core::application::use_cases::delete_user::DeleteUserUseCase,
    core::application::use_cases::find_user::FindUserUseCase,
    core::application::use_cases::list_user::ListUserUseCase,
    core::application::use_cases::update_user::UpdateUserUseCase,
    core::application::use_cases::update_user::UpdateUserUseCaseInputDto,
    core::infra::db::diesel::user::user_repository::UserDieselRepository, schema::user,
    shared::application::use_cases::use_case::UseCase,
    shared::webserver::errors::webservice_error::WebserviceError,
    shared::webserver::types::response::WebserviceResponse,
    webserver::config::database::Database,
};
use actix_session::Session;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};

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
) -> WebserviceResponse {
    let pool = Database::get_pool();
    let user_repository = UserDieselRepository::new(pool.get().unwrap(), user::table);
    let mut update_user = UpdateUserUseCase::new(session, Box::new(user_repository));

    let result = update_user.execute(payload.0);

    match result {
        Ok(result_data) => Ok(HttpResponse::Ok().json(result_data)),
        Err(message) => Err(message),
    }
}

#[get("/list")]
pub async fn list(session: Session) -> WebserviceResponse {
    let pool = Database::get_pool();
    let user_repository = UserDieselRepository::new(pool.get().unwrap(), user::table);
    let mut list_users = ListUserUseCase::new(session, Box::new(user_repository));

    let users = list_users.execute(());

    match users {
        Ok(users) => Ok(HttpResponse::Ok().json(users)),
        Err(message) => Err(message),
    }
}

#[get("/find-user/{id}")]
pub async fn find(session: Session, path: web::Path<String>) -> WebserviceResponse {
    let pool = Database::get_pool();
    let user_repository = UserDieselRepository::new(pool.get().unwrap(), user::table);
    let mut find_user = FindUserUseCase::new(session, Box::new(user_repository));

    let id = path.into_inner();

    if id.is_empty() {
        return Err(WebserviceError::BadRequest("Id is required".to_string()));
    }

    let user = find_user.execute(id);

    match user {
        Ok(user) => Ok(HttpResponse::Ok().json(user)),
        Err(message) => Err(message),
    }
}

#[delete("/delete/{id}")]
pub async fn delete(session: Session, path: web::Path<String>) -> WebserviceResponse {
    let pool = Database::get_pool();
    let user_repository = UserDieselRepository::new(pool.get().unwrap(), user::table);
    let mut delete_user = DeleteUserUseCase::new(session, Box::new(user_repository));

    let id = path.into_inner();

    if id.is_empty() {
        return Err(WebserviceError::BadRequest("Id is required".to_string()));
    }

    let delete_result = delete_user.execute(id);

    match delete_result {
        Ok(_) => Ok(HttpResponse::Ok().body("OK")),
        Err(error) => Err(error),
    }
}

pub fn config_routes(cfg: &mut web::ServiceConfig) {
    let scope = web::scope("/user")
        .service(sign_up)
        .service(update)
        .service(delete)
        .service(list)
        .service(find);

    cfg.service(scope);
}
