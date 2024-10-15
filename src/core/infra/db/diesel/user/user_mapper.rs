use uuid::Uuid;

use super::user_model::UserModel;
use crate::core::domain::entities::user::{User, UserCredentials};

pub struct UserMapper {}

impl UserMapper {
    pub fn to_entity(user_model: &UserModel) -> User {
        User::new(
            Some(Uuid::parse_str(&user_model.id).unwrap()),
            None,
            user_model.password_hash.clone(),
        )
    }
}
