use serde::{Deserialize, Serialize};
use serde_json::Value;
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

#[derive(Serialize)]
pub struct ViewUser {
    pub id: Option<Uuid>,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

#[derive(Serialize)]
pub struct UserProperties {
    pub id: Option<Uuid>,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password_hash: String,
    pub is_authenticated: bool,
    pub ask_for_new_password: bool,
}

#[derive(Serialize)]
pub struct User {
    id: Option<Uuid>,
    username: String,
    first_name: String,
    last_name: String,
    email: String,
    password_hash: String,
    is_authenticated: bool,
    ask_for_new_password: bool,
}

impl User {
    pub fn new(
        id: Option<Uuid>,
        username: String,
        first_name: String,
        last_name: String,
        email: String,
        password_hash: String,
        ask_for_new_password: bool,
    ) -> User {
        User {
            id: id.or(Some(Uuid::new_v4())),
            username,
            first_name,
            last_name,
            email,
            password_hash,
            is_authenticated: false,
            ask_for_new_password,
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

    pub fn get_properties(&self) -> UserProperties {
        UserProperties {
            id: self.id,
            username: self.username.clone(),
            first_name: self.first_name.clone(),
            last_name: self.last_name.clone(),
            email: self.email.clone(),
            password_hash: self.password_hash.clone(),
            is_authenticated: self.is_authenticated,
            ask_for_new_password: self.ask_for_new_password,
        }
    }

    pub fn get_view(&self) -> ViewUser {
        ViewUser {
            id: self.id,
            username: self.username.clone(),
            first_name: self.first_name.clone(),
            last_name: self.last_name.clone(),
            email: self.email.clone(),
        }
    }
}
