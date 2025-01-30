use actix_session::Session;

use crate::core::domain::repository::user_repository::UserRepository;
use crate::shared::application::use_cases::use_case::UseCase;
use crate::shared::webserver::errors::webservice_error::WebserviceError;

pub struct DeleteUserUseCase {
    session: Session,
    user_repository: Box<dyn UserRepository + 'static>,
}

impl DeleteUserUseCase {
    pub fn new(
        session: Session,
        user_repository: Box<dyn UserRepository + 'static>,
    ) -> DeleteUserUseCase {
        DeleteUserUseCase {
            session,
            user_repository,
        }
    }
}

impl UseCase<String, Result<(), WebserviceError>> for DeleteUserUseCase {
    fn execute(&mut self, id: String) -> Result<(), WebserviceError> {
        let uuid = uuid::Uuid::parse_str(&id).unwrap();
        let user_result = self.user_repository.find_by_id(uuid);

        if user_result.is_err() {
            return Err(WebserviceError::InternalServerError(
                "Error finding user".to_string(),
            ));
        }

        let user = user_result.unwrap();

        if user.is_none() {
            return Err(WebserviceError::NotFound("User not found".to_string()));
        }

        let delete_result = self.user_repository.delete(uuid);

        return match delete_result {
            Ok(_) => Ok(()),
            Err(_) => Err(WebserviceError::InternalServerError(
                "Error deleting user".to_string(),
            )),
        };
    }
}
