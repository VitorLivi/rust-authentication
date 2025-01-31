use crate::core::domain::entities::permission::Permission;
use crate::core::domain::repository::permission_repository::PermissionRepository;
use crate::shared::application::use_cases::use_case::UseCase;
use crate::shared::webserver::errors::webservice_error::WebserviceError;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct UpdatePermissionUseCaseInputDto {
    pub id: Uuid,
    pub name: String,
}

impl UpdatePermissionUseCaseInputDto {
    pub fn new(id: Uuid, name: String) -> UpdatePermissionUseCaseInputDto {
        UpdatePermissionUseCaseInputDto { id, name }
    }
}

pub struct UpdatePermissionUseCase {
    permission_repository: Box<dyn PermissionRepository + 'static>,
}

impl UpdatePermissionUseCase {
    pub fn new(
        permission_repository: Box<dyn PermissionRepository + 'static>,
    ) -> UpdatePermissionUseCase {
        UpdatePermissionUseCase {
            permission_repository,
        }
    }
}

impl UseCase<UpdatePermissionUseCaseInputDto, Result<(), WebserviceError>>
    for UpdatePermissionUseCase
{
    fn execute(&mut self, input: UpdatePermissionUseCaseInputDto) -> Result<(), WebserviceError> {
        let permission = Permission::new(Some(input.id), input.name.clone());
        let update_result = self.permission_repository.save(permission);

        match update_result {
            Ok(_) => Ok(()),
            Err(_) => Err(WebserviceError::InternalServerError(
                "Error updating permission".to_string(),
            )),
        }

    }
}
