use crate::core::application::use_cases::create_group::{
    CreateGroupUseCase, CreateGroupUseCaseInputDto,
};

use crate::core::application::use_cases::update_group::{
    UpdateGroupUseCase, UpdateGroupUseCaseInputDto,
};

use crate::core::application::use_cases::delete_group::{
    DeleteGroupUseCase, DeleteGroupUseCaseInputDto,
};

use crate::shared::application::use_cases::use_case::UseCase;

use crate::core::application::use_cases::find_group::FindGroupUseCase;
use crate::core::application::use_cases::list_group::ListGroupUseCase;

use crate::core::infra::db::diesel::group::group_repository::GroupDieselRepository;
use crate::schema::group;

use crate::shared::webserver::errors::webservice_error::WebserviceError;
use crate::shared::webserver::types::response::WebserviceResponse;
use crate::webserver::config::database::Database;

use actix_web::{delete, get, post, put, web, HttpResponse};

#[post("/create")]
pub async fn create(payload: web::Json<CreateGroupUseCaseInputDto>) -> WebserviceResponse {
    let pool = Database::get_pool();
    let group_repository = GroupDieselRepository::new(pool.get().unwrap(), group::table);
    let mut create_group = CreateGroupUseCase::new(Box::new(group_repository));

    let create_group_result = create_group.execute(payload.0);

    match create_group_result {
        Ok(_) => Ok(HttpResponse::Ok().body("OK")),
        Err(err) => Err(err),
    }
}

#[put("/update")]
pub async fn update(payload: web::Json<UpdateGroupUseCaseInputDto>) -> WebserviceResponse {
    let pool = Database::get_pool();
    let group_repository = GroupDieselRepository::new(pool.get().unwrap(), group::table);
    let mut update_group = UpdateGroupUseCase::new(Box::new(group_repository));

    let result = update_group.execute(payload.0);

    match result {
        Ok(result_data) => Ok(HttpResponse::Ok().json(result_data)),
        Err(message) => Err(message),
    }
}

#[get("/list")]
pub async fn list() -> WebserviceResponse {
    let pool = Database::get_pool();
    let group_repository = GroupDieselRepository::new(pool.get().unwrap(), group::table);
    let mut list_groups = ListGroupUseCase::new(Box::new(group_repository));

    let groups = list_groups.execute(());

    match groups {
        Ok(groups) => Ok(HttpResponse::Ok().json(groups)),
        Err(message) => Err(message),
    }
}

#[get("/find/{id}")]
pub async fn find(path: web::Path<String>) -> WebserviceResponse {
    let pool = Database::get_pool();
    let group_repository = GroupDieselRepository::new(pool.get().unwrap(), group::table);

    let id = path.into_inner();

    if id.is_empty() {
        return Err(WebserviceError::BadRequest("Id is required".to_string()));
    }

    let mut find_group = FindGroupUseCase::new(Box::new(group_repository));
    let group = find_group.execute(id);

    match group {
        Ok(group) => Ok(HttpResponse::Ok().json(group)),
        Err(message) => Err(message),
    }
}

#[delete("/delete/{id}")]
pub async fn delete(path: web::Path<String>) -> WebserviceResponse {
    let pool = Database::get_pool();
    let group_repository = GroupDieselRepository::new(pool.get().unwrap(), group::table);
    let mut delete_group = DeleteGroupUseCase::new(Box::new(group_repository));

    let id = path.into_inner();

    if id.is_empty() {
        return Err(WebserviceError::BadRequest("Id is required".to_string()));
    }

    let uuid = uuid::Uuid::parse_str(&id).unwrap();
    let delete_result = delete_group.execute(DeleteGroupUseCaseInputDto { id: uuid });

    match delete_result {
        Ok(_) => Ok(HttpResponse::Ok().body("OK")),
        Err(error) => Err(error),
    }
}

pub fn config_routes(cfg: &mut web::ServiceConfig) {
    let scope = web::scope("/group")
        .service(create)
        .service(update)
        .service(delete)
        .service(list)
        .service(find);

    cfg.service(scope);
}
