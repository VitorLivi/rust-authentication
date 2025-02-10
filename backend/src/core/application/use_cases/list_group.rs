use crate::core::domain::entities::group::Group;
use crate::core::domain::repository::group_repository::GroupRepository;
use crate::shared::application::use_cases::use_case::UseCase;
use crate::shared::webserver::errors::webservice_error::WebserviceError;

pub struct ListGroupUseCase {
    group_repository: Box<dyn GroupRepository + 'static>,
}

impl ListGroupUseCase {
    pub fn new(group_repository: Box<dyn GroupRepository + 'static>) -> ListGroupUseCase {
        ListGroupUseCase { group_repository }
    }
}

impl UseCase<(), Result<Vec<Group>, WebserviceError>> for ListGroupUseCase {
    fn execute(&mut self, _input: ()) -> Result<Vec<Group>, WebserviceError> {
        let find_result = self.group_repository.find_all();

        match find_result {
            Ok(groups) => {
                return Ok(groups);
            }
            Err(_) => {
                return Err(WebserviceError::InternalServerError(
                    "Error listing groups".to_string(),
                ))
            }
        }
    }
}
