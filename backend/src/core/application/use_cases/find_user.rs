use actix_session::Session;

use crate::core::domain::entities::user::ViewUser;
use crate::core::domain::repository::user_repository::UserRepository;
use crate::shared::application::use_cases::use_case::UseCase;
use crate::shared::webserver::errors::webservice_error::WebserviceError;

pub struct FindUserUseCase {
    session: Session,
    user_repository: Box<dyn UserRepository + 'static>,
}

impl FindUserUseCase {
    pub fn new(
        session: Session,
        user_repository: Box<dyn UserRepository + 'static>,
    ) -> FindUserUseCase {
        FindUserUseCase {
            session,
            user_repository,
        }
    }
}

impl UseCase<String, Result<ViewUser, WebserviceError>> for FindUserUseCase {
    fn execute(&mut self, id: String) -> Result<ViewUser, WebserviceError> {
        let user_result = self
            .user_repository
            .find_by_id(uuid::Uuid::parse_str(&id).unwrap());

        match user_result {
            Ok(Some(user)) => Ok(user.get_view()),
            Ok(None) => Err(WebserviceError::NotFound("User not found".to_string())),
            Err(_) => Err(WebserviceError::InternalServerError(
                "Error finding user".to_string(),
            )),
        }
    }
}
