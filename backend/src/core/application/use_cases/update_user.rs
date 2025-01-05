use actix_session::Session;
use serde::Deserialize;

use crate::core::domain::entities::authenticator::Authenticator;
use crate::core::domain::entities::user::User;
use crate::core::domain::entities::user::UserUpdatableProperties;
use crate::core::domain::repository::user_repository::UserRepository;
use crate::shared::application::use_cases::use_case::UseCase;
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

impl UseCase<UpdateUserUseCaseInputDto, Result<User, &'static str>> for UpdateUserUseCase {
    fn execute(&mut self, input: UpdateUserUseCaseInputDto) -> Result<User, &'static str> {
        let find_result: Option<User> = self
            .user_repository
            .find_by_id(uuid::Uuid::parse_str(&input.id).unwrap());

        if find_result.is_none() {
            println!("User not found");
            return Err("User not found");
        }

        let mut found_user = find_result.unwrap();
        let mut string_birth_date = input.birth_date.unwrap();
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

        let result = self.user_repository.save(found_user);
        return Ok(result.unwrap());
    }
}
