use super::user_model::UserModel;
use crate::core::domain::entities::user::User;

pub struct UserMapper {}

impl UserMapper {
    pub fn to_entity(user_model: &UserModel) -> User {
        User::new(Some(user_model.id), None, user_model.password_hash.clone())
    }
}
