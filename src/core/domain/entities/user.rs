use std::collections::HashMap;

use uuid::Uuid;

use crate::core::domain::entities::authenticator::Authenticator;

pub struct UserCredentials {
    pub username: String,
    pub password: String,
}

impl UserCredentials {
    pub fn new(username: String, password: String) -> UserCredentials {
        UserCredentials { username, password }
    }
}

pub struct User {
    id: Option<Uuid>,
    username: String,
    first_name: String,
    last_name: String,
    email: String,
    password_hash: String,
    is_authenticated: bool,
}

impl User {
    pub fn new(
        id: Option<Uuid>,
        username: String,
        first_name: String,
        last_name: String,
        email: String,
        password_hash: String,
    ) -> User {
        User {
            id: id.or(Some(Uuid::new_v4())),
            username,
            first_name,
            last_name,
            email,
            password_hash,
            is_authenticated: false,
        }
    }

    pub fn get_id(&self) -> Option<Uuid> {
        return self.id;
    }

    pub fn authenticate(
        &mut self,
        authenticator: &mut Authenticator,
        user_credentials: &mut UserCredentials,
    ) {
        let is_authenticated = authenticator.authenticate(&self, user_credentials);

        if is_authenticated {
            self.is_authenticated = true;
        }

        return;
    }

    pub fn get_properties(&self) -> HashMap<String, String> {
        let mut properties = HashMap::new();

        properties.insert("id".to_string(), self.id.unwrap().to_string());
        properties.insert("username".to_string(), self.username.to_string());
        properties.insert("first_name".to_string(), self.first_name.to_string());
        properties.insert("last_name".to_string(), self.last_name.to_string());
        properties.insert("email".to_string(), self.email.to_string());
        properties.insert("password_hash".to_string(), self.password_hash.to_string());

        return properties;
    }
}
