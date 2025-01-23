use super::user_model::UserModel;
use crate::core::domain::entities::user::User;
use std::time::SystemTime;

pub struct UserMapper {}

impl UserMapper {
    pub fn to_entity(user_model: &UserModel) -> User {
        User::new(
            Some(user_model.id),
            user_model.username.clone().unwrap(),
            user_model.first_name.clone(),
            user_model.last_name.clone(),
            user_model.email.clone(),
            user_model.birth_date.clone(),
            user_model.password_hash.clone(),
            user_model.ask_for_new_password == 1,
        )
    }

    pub fn to_model(user: &User) -> UserModel {
        let user_props = user.get_properties();

        UserModel {
            id: user_props.id.unwrap(),
            email: user_props.email,
            birth_date: user_props.birth_date,
            created_at: SystemTime::now(),
            updated_at: SystemTime::now(),
            first_name: user_props.first_name,
            last_name: user_props.last_name,
            username: Some(user_props.username),
            password_hash: user_props.password_hash,
            status: None,
            ask_for_new_password: if user_props.ask_for_new_password == true {
                1
            } else {
                0
            },
        }
    }
}
