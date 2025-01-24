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
    webserver::config::database::Database,
};
use actix_web::{post, web, HttpResponse, Responder};

#[post("/create")]
pub async fn create(payload: web::Json<AddPermissionUseCaseInputDto>) -> impl Responder {
    let pool = Database::get_pool();
    let permission_repository =
        PermissionDieselRepository::new(pool.get().unwrap(), permission::table);
    let mut add_permission = AddPermissionUseCase::new(Box::new(permission_repository));

    let result = add_permission.execute(payload.0);

    match result {
        Ok(permission) => HttpResponse::Ok().json(permission),
        Err(message) => {
            if message == "Permission already exists" {
                HttpResponse::Conflict().body(message)
            } else {
                HttpResponse::BadRequest().body(message)
            }
        }
    }
}

#[post("/update")]
pub async fn update(payload: web::Json<UpdatePermissionUseCaseInputDto>) -> impl Responder {
    let pool = Database::get_pool();
    let permission_repository =
        PermissionDieselRepository::new(pool.get().unwrap(), permission::table);
    let mut update_permission = UpdatePermissionUseCase::new(Box::new(permission_repository));

    let result = update_permission.execute(payload.0);

    match result {
        Ok(_) => HttpResponse::Ok().body("OK"),
        Err(message) => HttpResponse::BadRequest().body(message),
    }
}

#[post("/delete")]
pub async fn delete(payload: web::Json<DeletePermissionUseCaseInputDto>) -> impl Responder {
    let pool = Database::get_pool();
    let permission_repository =
        PermissionDieselRepository::new(pool.get().unwrap(), permission::table);
    let mut delete_permission = DeletePermissionUseCase::new(Box::new(permission_repository));

    let result = delete_permission.execute(payload.0);

    match result {
        Ok(_) => HttpResponse::Ok().body("OK"),
        Err(message) => HttpResponse::BadRequest().body(message),
    }
}

pub fn config_routes(cfg: &mut web::ServiceConfig) {
    let scope = web::scope("/permission")
        .service(create)
        .service(delete)
        .service(update);

    cfg.service(scope);
}
