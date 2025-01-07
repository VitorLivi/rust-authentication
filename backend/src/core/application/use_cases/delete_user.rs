use actix_session::Session;
use serde::Deserialize;

use crate::core::domain::entities::authenticator::Authenticator;
use crate::core::domain::entities::user::User;
use crate::core::domain::repository::user_repository::UserRepository;
use crate::shared::application::use_cases::use_case::UseCase;
use chrono::NaiveDate;

pub struct DeleteUserUseCase {
    session: Session,
    user_repository: Box<dyn UserRepository + 'static>,
}

impl DeleteUserUseCase {
    pub fn new(
        session: Session,
        user_repository: Box<dyn UserRepository + 'static>,
    ) -> DeleteUserUseCase {
        DeleteUserUseCase {
            session,
            user_repository,
        }
    }
}

impl UseCase<String, Result<(), &'static str>> for DeleteUserUseCase {
    fn execute(&mut self, id: String) -> Result<(), &'static str> {
        let uuid = uuid::Uuid::parse_str(&id).unwrap();
        let found_user = self.user_repository.find_by_id(uuid);

        if found_user.is_none() {
            return Err("User not found");
        }

        println!("FOUND USER: {:?}", found_user.unwrap().get_id());

        let delete_result = self.user_repository.delete(uuid);

        return match delete_result {
            Ok(_) => Ok(()),
            Err(_) => Err("Error deleting user"),
        }
    }
}
