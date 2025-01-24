use actix_session::Session;
use serde::Deserialize;

use crate::core::domain::entities::permission::Permission;
use crate::core::domain::repository::permission_repository::PermissionRepository;
use crate::shared::application::use_cases::use_case::UseCase;

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

impl UseCase<AddPermissionUseCaseInputDto, Result<(), String>> for AddPermissionUseCase {
    fn execute(&mut self, input: AddPermissionUseCaseInputDto) -> Result<(), String> {
        let permission = Permission::new(None, input.name.clone());
        let save_result = self.permission_repository.save(permission);

        if save_result.is_err() {
            return Err(save_result.err().unwrap());
        }

        Ok(())
    }
}
