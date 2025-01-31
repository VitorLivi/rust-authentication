use actix_session::Session;
use serde::Deserialize;

use crate::core::domain::entities::user::User;
use crate::core::domain::entities::user::UserUpdatableProperties;
use crate::core::domain::repository::user_repository::UserRepository;
use crate::core::domain::services::authenticator::Authenticator;
use crate::shared::application::use_cases::use_case::UseCase;
use crate::shared::webserver::errors::webservice_error::WebserviceError;
use chrono::NaiveDate;

#[derive(Deserialize)]
pub struct UpdateUserUseCaseInputDto {
    id: String,
    ask_for_new_password: bool,
    email: String,
    birth_date: Option<String>,
    first_name: String,
    last_name: String,
    password: Option<String>,
    username: String,
}

impl UpdateUserUseCaseInputDto {
    pub fn new(
        id: String,
        ask_for_new_password: bool,
        email: String,
        birth_date: Option<String>,
        first_name: String,
        last_name: String,
        password: Option<String>,
        username: String,
    ) -> UpdateUserUseCaseInputDto {
        UpdateUserUseCaseInputDto {
            id,
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

pub struct UpdateUserUseCase {
    session: Session,
    user_repository: Box<dyn UserRepository + 'static>,
}

impl UpdateUserUseCase {
    pub fn new(
        session: Session,
        user_repository: Box<dyn UserRepository + 'static>,
    ) -> UpdateUserUseCase {
        UpdateUserUseCase {
            session,
            user_repository,
        }
    }
}

impl UseCase<UpdateUserUseCaseInputDto, Result<User, WebserviceError>> for UpdateUserUseCase {
    fn execute(&mut self, input: UpdateUserUseCaseInputDto) -> Result<User, WebserviceError> {
        let find_result = self
            .user_repository
            .find_by_id(uuid::Uuid::parse_str(&input.id).unwrap());

        match find_result {
            Ok(None) => return Err(WebserviceError::NotFound("User not found".to_string())),
            Err(_) => {
                return Err(WebserviceError::InternalServerError(
                    "Error finding user".to_string(),
                ))
            }
            _ => {}
        }

        let mut found_user = find_result.unwrap().unwrap();
        let string_birth_date = input.birth_date.unwrap();
        let mut birth_date: Option<NaiveDate> = None;

        if !string_birth_date.is_empty() {
            println!("Parsing date");

            birth_date = Some(NaiveDate::parse_from_str(&string_birth_date, "%Y-%m-%d").unwrap());
        }

        found_user.update(UserUpdatableProperties {
            username: Some(input.username),
            first_name: Some(input.first_name),
            last_name: Some(input.last_name),
            email: Some(input.email),
            birth_date,
            password_hash: input
                .password
                .map(|password| Authenticator::create_hash(&password)),
            ask_for_new_password: Some(input.ask_for_new_password),
        });

        let save_result = self.user_repository.save(found_user);

        match save_result {
            Ok(user) => Ok(user),
            Err(_) => {
                return Err(WebserviceError::InternalServerError(
                    "Error saving user".to_string(),
                ))
            }
        }
    }
}
