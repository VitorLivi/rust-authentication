use crate::core::domain::entities::user::ViewUser;
use crate::core::domain::repository::user_repository::UserRepository;
use crate::shared::application::use_cases::use_case::UseCase;
use crate::shared::webserver::errors::webservice_error::WebserviceError;
use actix_session::Session;

pub struct ListUserUseCase {
    session: Session,
    user_repository: Box<dyn UserRepository + 'static>,
}

impl ListUserUseCase {
    pub fn new(
        session: Session,
        user_repository: Box<dyn UserRepository + 'static>,
    ) -> ListUserUseCase {
        ListUserUseCase {
            session,
            user_repository,
        }
    }
}

impl UseCase<(), Result<Vec<ViewUser>, WebserviceError>> for ListUserUseCase {
    fn execute(&mut self, _input: ()) -> Result<Vec<ViewUser>, WebserviceError> {
        let find_result = self.user_repository.find_all();

        match find_result {
            Ok(users) => {
                return Ok(users.iter().map(|user| user.get_view()).collect());
            }
            Err(_) => {
                return Err(WebserviceError::InternalServerError(
                    "Error listing users".to_string(),
                ))
            }
        }
    }
}
