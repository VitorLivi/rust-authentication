use actix_session::Session;
use serde::Deserialize;

use crate::core::domain::services::authenticator::Authenticator;
use crate::core::domain::entities::user::UserCredentials;
use crate::core::domain::repository::user_repository::UserRepository;
use crate::shared::application::use_cases::use_case::UseCase;
use crate::shared::webserver::errors::webservice_error::WebserviceError;

#[derive(Deserialize)]
pub struct AuthenticateUserUseCaseInputDto {
    pub username: String,
    pub password: String,
}

impl AuthenticateUserUseCaseInputDto {
    pub fn new(username: String, password: String) -> AuthenticateUserUseCaseInputDto {
        AuthenticateUserUseCaseInputDto { username, password }
    }
}

pub struct AuthenticateUserUseCase {
    session: Session,
    user_repository: Box<dyn UserRepository + 'static>,
}

impl AuthenticateUserUseCase {
    pub fn new(
        session: Session,
        user_repository: Box<dyn UserRepository + 'static>,
    ) -> AuthenticateUserUseCase {
        AuthenticateUserUseCase {
            user_repository,
            session,
        }
    }
}

impl UseCase<AuthenticateUserUseCaseInputDto, Result<(), WebserviceError>>
    for AuthenticateUserUseCase
{
    fn execute(&mut self, input: AuthenticateUserUseCaseInputDto) -> Result<(), WebserviceError> {
        let mut authenticator = Authenticator::new(&self.session);

        let result = self
            .user_repository
            .find_by_username(input.username.clone());

        if result.is_err() {
            return Err(WebserviceError::NotFound("User not found".to_string()));
        }

        let user_credentials = &mut UserCredentials::new(input.username, input.password);

        let mut user = result.unwrap();
        user.authenticate(&mut authenticator, user_credentials);

        Ok(())
    }
}
