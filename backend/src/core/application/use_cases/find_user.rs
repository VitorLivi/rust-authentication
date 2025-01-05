use actix_session::Session;
use serde::Deserialize;

use crate::core::domain::entities::authenticator::Authenticator;
use crate::core::domain::entities::user::{User, ViewUser};
use crate::core::domain::repository::user_repository::UserRepository;
use crate::shared::application::use_cases::use_case::UseCase;

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

impl UseCase<String, Result<ViewUser, &'static str>> for FindUserUseCase {
    fn execute(&mut self, id: String) -> Result<ViewUser, &'static str> {
        let user = self
            .user_repository
            .find_by_id(uuid::Uuid::parse_str(&id).unwrap());

        if user.is_none() {
            return Err("User not found");
        }

        Ok(user.unwrap().get_view())
    }
}
