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

        if user_result.is_err() {
            return Err(WebserviceError::InternalServerError(
                "Error finding user".to_string(),
            ));
        }

        let user = user_result.unwrap();

        if user.is_none() {
            return Err(WebserviceError::NotFound("User not found".to_string()));
        }

        Ok(user.unwrap().get_view())
    }
}
