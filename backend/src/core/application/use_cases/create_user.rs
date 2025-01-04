use actix_session::Session;
use serde::Deserialize;

use crate::core::domain::entities::authenticator::Authenticator;
use crate::core::domain::entities::user::User;
use crate::core::domain::repository::user_repository::UserRepository;
use crate::shared::application::use_cases::use_case::UseCase;

#[derive(Deserialize)]
pub struct CreateUserUseCaseInputDto {
    ask_for_new_password: bool,
    birth_date: String,
    email: String,
    first_name: String,
    last_name: String,
    password: String,
    username: String,
}

impl CreateUserUseCaseInputDto {
    pub fn new(
        ask_for_new_password: bool,
        birth_date: String,
        email: String,
        first_name: String,
        last_name: String,
        password: String,
        username: String,
    ) -> CreateUserUseCaseInputDto {
        CreateUserUseCaseInputDto {
            ask_for_new_password,
            birth_date,
            email,
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

impl UseCase<CreateUserUseCaseInputDto, User> for CreateUserUseCase {
    fn execute(&mut self, input: CreateUserUseCaseInputDto) -> User {
        let password_hash = Authenticator::create_hash(&input.password);
        let user = User::new(
            None,
            input.username,
            input.first_name,
            input.last_name,
            input.email,
            password_hash,
            input.ask_for_new_password,
        );

        let result = self.user_repository.save(user);

        return result.unwrap();
    }
}
