use crate::core::domain::entities::group::Group;
use crate::core::domain::repository::group_repository::GroupRepository;
use crate::shared::application::use_cases::use_case::UseCase;
use crate::shared::webserver::errors::webservice_error::WebserviceError;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct UpdateGroupUseCaseInputDto {
    pub id: Uuid,
    pub name: String,
}

impl UpdateGroupUseCaseInputDto {
    pub fn new(id: Uuid, name: String) -> UpdateGroupUseCaseInputDto {
        UpdateGroupUseCaseInputDto { id, name }
    }
}

pub struct UpdateGroupUseCase {
    group_repository: Box<dyn GroupRepository + 'static>,
}

impl UpdateGroupUseCase {
    pub fn new(group_repository: Box<dyn GroupRepository + 'static>) -> UpdateGroupUseCase {
        UpdateGroupUseCase { group_repository }
    }
}

impl UseCase<UpdateGroupUseCaseInputDto, Result<(), WebserviceError>> for UpdateGroupUseCase {
    fn execute(&mut self, input: UpdateGroupUseCaseInputDto) -> Result<(), WebserviceError> {
        let group = Group::new(Some(input.id), input.name.clone());
        let update_result = self.group_repository.save(group);

        match update_result {
            Ok(_) => Ok(()),
            Err(_) => Err(WebserviceError::InternalServerError(
                "Error updating group".to_string(),
            )),
        }
    }
}
