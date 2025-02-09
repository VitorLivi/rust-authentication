use crate::{
    core::{
        application::use_cases::add_permission::{
            AddPermissionUseCase, AddPermissionUseCaseInputDto,
        },
        application::use_cases::delete_permission::{
            DeletePermissionUseCase, DeletePermissionUseCaseInputDto,
        },
        application::use_cases::update_permission::{
            UpdatePermissionUseCase, UpdatePermissionUseCaseInputDto,
        },
        infra::db::diesel::permission::permission_repository::PermissionDieselRepository,
    },
    schema::permission,
    shared::application::use_cases::use_case::UseCase,
    shared::webserver::types::response::WebserviceResponse,
    webserver::config::database::Database,
};
use actix_web::{post, web, HttpResponse};

#[post("/create")]
pub async fn create(payload: web::Json<AddPermissionUseCaseInputDto>) -> WebserviceResponse {
    let pool = Database::get_pool();
    let permission_repository =
        PermissionDieselRepository::new(pool.get().unwrap(), permission::table);
    let mut add_permission = AddPermissionUseCase::new(Box::new(permission_repository));

    let result = add_permission.execute(payload.0);

    match result {
        Ok(permission) => Ok(HttpResponse::Ok().json(permission)),
        Err(err) => Err(err),
    }
}

#[post("/update")]
pub async fn update(payload: web::Json<UpdatePermissionUseCaseInputDto>) -> WebserviceResponse {
    let pool = Database::get_pool();
    let permission_repository =
        PermissionDieselRepository::new(pool.get().unwrap(), permission::table);
    let mut update_permission = UpdatePermissionUseCase::new(Box::new(permission_repository));

    let result = update_permission.execute(payload.0);

    match result {
        Ok(_) => Ok(HttpResponse::Ok().body("OK")),
        Err(err) => Err(err),
    }
}

#[post("/delete")]
pub async fn delete(payload: web::Json<DeletePermissionUseCaseInputDto>) -> WebserviceResponse {
    let pool = Database::get_pool();
    let permission_repository =
        PermissionDieselRepository::new(pool.get().unwrap(), permission::table);
    let mut delete_permission = DeletePermissionUseCase::new(Box::new(permission_repository));

    let result = delete_permission.execute(payload.0);

    match result {
        Ok(_) => Ok(HttpResponse::Ok().body("OK")),
        Err(err) => Err(err),
    }
}

pub fn config_routes(cfg: &mut web::ServiceConfig) {
    let scope = web::scope("/permission")
        .service(create)
        .service(delete)
        .service(update);

    cfg.service(scope);
}
