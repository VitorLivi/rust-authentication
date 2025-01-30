use actix_session::Session;
use serde::Deserialize;

use crate::core::domain::services::authenticator::Authenticator;
use crate::core::domain::entities::user::User;
use crate::core::domain::repository::user_repository::UserRepository;
use crate::shared::application::use_cases::use_case::UseCase;
use crate::shared::webserver::errors::webservice_error::WebserviceError;
use chrono::NaiveDate;

#[derive(Deserialize)]
pub struct CreateUserUseCaseInputDto {
    ask_for_new_password: bool,
    email: String,
    birth_date: Option<NaiveDate>,
    first_name: String,
    last_name: String,
    password: String,
    username: String,
}

impl CreateUserUseCaseInputDto {
    pub fn new(
        ask_for_new_password: bool,
        email: String,
        birth_date: Option<NaiveDate>,
        first_name: String,
        last_name: String,
        password: String,
        username: String,
    ) -> CreateUserUseCaseInputDto {
        CreateUserUseCaseInputDto {
            ask_for_new_password,
            email,
            birth_date,
            first_name,
            last_name,
            password,
            username,
        }
    }
}

pub struct CreateUserUseCase {
    session: Session,
    user_repository: Box<dyn UserRepository + 'static>,
}

impl CreateUserUseCase {
    pub fn new(
        session: Session,
        user_repository: Box<dyn UserRepository + 'static>,
    ) -> CreateUserUseCase {
        CreateUserUseCase {
            session,
            user_repository,
        }
    }
}

impl UseCase<CreateUserUseCaseInputDto, Result<User, WebserviceError>> for CreateUserUseCase {
    fn execute(&mut self, input: CreateUserUseCaseInputDto) -> Result<User, WebserviceError> {
        let password_hash = Authenticator::create_hash(&input.password);
        let user = User::new(
            None,
            input.username,
            input.first_name,
            input.last_name,
            input.email,
            input.birth_date,
            password_hash,
            input.ask_for_new_password,
        );

        let result = self.user_repository.save(user);

        if result.is_err() {
            return Err(WebserviceError::InternalServerError(
                "Error saving user".to_string(),
            ));
        }

        return Ok(result.unwrap());
    }
}
