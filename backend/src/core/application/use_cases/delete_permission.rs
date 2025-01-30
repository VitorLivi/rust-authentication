use serde::Deserialize;

use crate::core::domain::repository::permission_repository::PermissionRepository;
use crate::shared::application::use_cases::use_case::UseCase;
use crate::shared::webserver::errors::webservice_error::WebserviceError;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct DeletePermissionUseCaseInputDto {
    pub id: Uuid,
}

impl DeletePermissionUseCaseInputDto {
    pub fn new(id: Uuid) -> DeletePermissionUseCaseInputDto {
        DeletePermissionUseCaseInputDto { id }
    }
}

pub struct DeletePermissionUseCase {
    permission_repository: Box<dyn PermissionRepository + 'static>,
}

impl DeletePermissionUseCase {
    pub fn new(
        permission_repository: Box<dyn PermissionRepository + 'static>,
    ) -> DeletePermissionUseCase {
        DeletePermissionUseCase {
            permission_repository,
        }
    }
}

impl UseCase<DeletePermissionUseCaseInputDto, Result<(), WebserviceError>>
    for DeletePermissionUseCase
{
    fn execute(&mut self, input: DeletePermissionUseCaseInputDto) -> Result<(), WebserviceError> {
        let delete_result = self.permission_repository.delete(input.id);

        if delete_result.is_err() {
            return Err(WebserviceError::InternalServerError(
                "Error deleting permission".to_string(),
            ));
        }

        Ok(())
    }
}
