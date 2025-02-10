use serde::Deserialize;

use crate::core::domain::repository::group_repository::GroupRepository;
use crate::shared::application::use_cases::use_case::UseCase;
use crate::shared::webserver::errors::webservice_error::WebserviceError;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct DeleteGroupUseCaseInputDto {
    pub id: Uuid,
}

impl DeleteGroupUseCaseInputDto {
    pub fn new(id: Uuid) -> DeleteGroupUseCaseInputDto {
        DeleteGroupUseCaseInputDto { id }
    }
}

pub struct DeleteGroupUseCase {
    group_repository: Box<dyn GroupRepository + 'static>,
}

impl DeleteGroupUseCase {
    pub fn new(group_repository: Box<dyn GroupRepository + 'static>) -> DeleteGroupUseCase {
        DeleteGroupUseCase { group_repository }
    }
}

impl UseCase<DeleteGroupUseCaseInputDto, Result<(), WebserviceError>> for DeleteGroupUseCase {
    fn execute(&mut self, input: DeleteGroupUseCaseInputDto) -> Result<(), WebserviceError> {
        let delete_result = self.group_repository.delete(input.id);

        match delete_result {
            Ok(_) => Ok(()),
            Err(_) => {
                return Err(WebserviceError::InternalServerError(
                    "Error deleting group".to_string(),
                ));
            }
        }
    }
}
