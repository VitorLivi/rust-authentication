use crate::{
    core::{application::use_cases::add_permission::{AddPermissionUseCase, AddPermissionUseCaseInputDto}, domain::repository::permission_repository, infra::db::diesel::permission::permission_repository::PermissionDieselRepository}, schema::permission, shared::application::use_cases::use_case::UseCase, webserver::config::database::Database
};
use actix_session::Session;
use actix_web::{post, web, HttpResponse, Responder};

#[post("/create")]
pub async fn create(session: Session, payload: web::Json<AddPermissionUseCaseInputDto>) -> impl Responder {
    let pool = Database::get_pool();
    let permission_repository = PermissionDieselRepository::new(pool.get().unwrap(), permission::table);
    let mut add_permission = AddPermissionUseCase::new(session, Box::new(permission_repository));

    add_permission.execute(payload.0);
    HttpResponse::Ok().body("OK")
}

pub fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(create);
}
