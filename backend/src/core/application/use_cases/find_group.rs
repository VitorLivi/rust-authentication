use crate::core::domain::entities::group::Group;
use crate::core::domain::repository::group_repository::GroupRepository;
use crate::shared::application::use_cases::use_case::UseCase;
use crate::shared::webserver::errors::webservice_error::WebserviceError;

pub struct FindGroupUseCase {
    group_repository: Box<dyn GroupRepository + 'static>,
}

impl FindGroupUseCase {
    pub fn new(group_repository: Box<dyn GroupRepository + 'static>) -> FindGroupUseCase {
        FindGroupUseCase { group_repository }
    }
}

impl UseCase<String, Result<Group, WebserviceError>> for FindGroupUseCase {
    fn execute(&mut self, id: String) -> Result<Group, WebserviceError> {
        let group_result = self
            .group_repository
            .find_by_id(uuid::Uuid::parse_str(&id).unwrap());

        match group_result {
            Ok(Some(group)) => Ok(group),
            Ok(None) => Err(WebserviceError::NotFound("Group not found".to_string())),
            Err(_) => Err(WebserviceError::InternalServerError(
                "Error finding group".to_string(),
            )),
        }
    }
}
