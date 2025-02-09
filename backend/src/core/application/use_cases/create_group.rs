use serde::Deserialize;

use crate::core::domain::entities::group::Group;
use crate::core::domain::repository::group_repository::GroupRepository;
use crate::shared::application::use_cases::use_case::UseCase;
use crate::shared::webserver::errors::webservice_error::WebserviceError;

#[derive(Deserialize)]
pub struct CreateGroupUseCaseInputDto {
    pub name: String,
}

impl CreateGroupUseCaseInputDto {
    pub fn new(name: String) -> CreateGroupUseCaseInputDto {
        CreateGroupUseCaseInputDto { name }
    }
}

pub struct CreateGroupUseCase {
    permission_repository: Box<dyn GroupRepository + 'static>,
}

impl CreateGroupUseCase {
    pub fn new(permission_repository: Box<dyn GroupRepository + 'static>) -> CreateGroupUseCase {
        CreateGroupUseCase {
            permission_repository,
        }
    }
}

impl UseCase<CreateGroupUseCaseInputDto, Result<(), WebserviceError>> for CreateGroupUseCase {
    fn execute(&mut self, input: CreateGroupUseCaseInputDto) -> Result<(), WebserviceError> {
        let group = Group::new(None, input.name.clone());
        let result = self.permission_repository.save(group);

        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(WebserviceError::InternalServerError(
                "Error saving group".to_string(),
            )),
        }
    }
}
