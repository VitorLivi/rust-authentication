use serde::Deserialize;

use crate::core::domain::entities::permission::Permission;
use crate::core::domain::repository::permission_repository::PermissionRepository;
use crate::shared::application::use_cases::use_case::UseCase;
use crate::shared::webserver::errors::webservice_error::WebserviceError;

#[derive(Deserialize)]
pub struct AddPermissionUseCaseInputDto {
    pub name: String,
}

impl AddPermissionUseCaseInputDto {
    pub fn new(name: String) -> AddPermissionUseCaseInputDto {
        AddPermissionUseCaseInputDto { name }
    }
}

pub struct AddPermissionUseCase {
    permission_repository: Box<dyn PermissionRepository + 'static>,
}

impl AddPermissionUseCase {
    pub fn new(
        permission_repository: Box<dyn PermissionRepository + 'static>,
    ) -> AddPermissionUseCase {
        AddPermissionUseCase {
            permission_repository,
        }
    }
}

impl UseCase<AddPermissionUseCaseInputDto, Result<(), WebserviceError>> for AddPermissionUseCase {
    fn execute(&mut self, input: AddPermissionUseCaseInputDto) -> Result<(), WebserviceError> {
        let permission = Permission::new(None, input.name.clone());
        let result = self.permission_repository.save(permission);

        if result.is_err() {
            return Err(WebserviceError::InternalServerError(
                "Error saving permission".to_string(),
            ));
        }

        Ok(())
    }
}
