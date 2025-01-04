use actix_session::Session;
use serde::Deserialize;

use crate::core::domain::entities::authenticator::Authenticator;
use crate::core::domain::entities::user::{User, ViewUser};
use crate::core::domain::repository::user_repository::UserRepository;
use crate::shared::application::use_cases::use_case::UseCase;

pub struct ListUserUseCase {
    session: Session,
    user_repository: Box<dyn UserRepository + 'static>,
}

impl ListUserUseCase {
    pub fn new(
        session: Session,
        user_repository: Box<dyn UserRepository + 'static>,
    ) -> ListUserUseCase {
        ListUserUseCase {
            session,
            user_repository,
        }
    }
}

impl UseCase<(), Vec<ViewUser>> for ListUserUseCase {
    fn execute(&mut self, _input: ()) -> Vec<ViewUser> {
        let users = self.user_repository.find_all();

        return users.iter().map(|user| user.get_view()).collect();
    }
}
