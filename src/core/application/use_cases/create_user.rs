use actix_web::FromRequest;
use futures::FutureExt;
use serde::Deserialize;

use crate::core::domain::entities::authenticator::Authenticator;
use crate::core::domain::entities::user::User;
use crate::core::domain::repository::user_repository::UserRepository;
use crate::shared::application::use_cases::use_case::UseCase;

#[derive(Deserialize)]
pub struct CreateUserInputDto {
    pub username: String,
    pub password: String,
}

impl CreateUserInputDto {
    pub fn new(username: String, password: String) -> CreateUserInputDto {
        CreateUserInputDto { username, password }
    }
}

pub struct CreateUser {
    user_repository: Box<dyn UserRepository + 'static>,
}

impl CreateUser {
    pub fn new(user_repository: Box<dyn UserRepository + 'static>) -> CreateUser {
        CreateUser { user_repository }
    }
}

impl UseCase<CreateUserInputDto, ()> for CreateUser {
    fn execute(&self, input: CreateUserInputDto) -> () {
        let password_hash = Authenticator::create_hash(&input.password);
        let user = User::new(None, None, password_hash);

        self.user_repository.save(user);
    }
}
