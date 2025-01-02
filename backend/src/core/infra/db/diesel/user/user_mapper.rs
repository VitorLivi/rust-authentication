use super::user_model::UserModel;
use crate::core::domain::entities::user::User;

pub struct UserMapper {}

impl UserMapper {
    pub fn to_entity(user_model: &UserModel) -> User {
        User::new(
            Some(user_model.id),
            user_model.username.clone().unwrap(),
            user_model.first_name.clone(),
            user_model.last_name.clone(),
            user_model.email.clone(),
            user_model.password_hash.clone(),
            user_model.ask_reset_password == 1
        )
    }

    pub fn to_model(user: &User) -> UserModel {
        let user_props = user.get_properties();

        UserModel {
            id: user_props.get("id").unwrap().parse().unwrap(),
            email: user_props.get("email").unwrap().to_string(),
            created_at: None,
            updated_at: None,
            first_name: user_props.get("first_name").unwrap().to_string(),
            last_name: user_props.get("last_name").unwrap().to_string(),
            username: Some(user_props.get("username").unwrap().to_string()),
            password_hash: user_props.get("password_hash").unwrap().to_string(),
            status: None,
            ask_reset_password: user_props
                .get("ask_reset_password")
                .unwrap()
                .parse()
                .unwrap(),
        }
    }
}
