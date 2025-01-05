use chrono::NaiveDate;
use serde::Serialize;
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
    pub birth_date: Option<NaiveDate>,
    pub ask_for_new_password: bool,
}

#[derive(Serialize, Debug)]
pub struct UserProperties {
    pub id: Option<Uuid>,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub birth_date: Option<NaiveDate>,
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
    birth_date: Option<NaiveDate>,
    password_hash: String,
    is_authenticated: bool,
    ask_for_new_password: bool,
}

pub struct UserUpdatableProperties {
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub birth_date: Option<NaiveDate>,
    pub password_hash: Option<String>,
    pub ask_for_new_password: Option<bool>,
}

impl User {
    pub fn new(
        id: Option<Uuid>,
        username: String,
        first_name: String,
        last_name: String,
        email: String,
        birth_date: Option<NaiveDate>,
        password_hash: String,
        ask_for_new_password: bool,
    ) -> User {
        User {
            id: id.or(Some(Uuid::new_v4())),
            username,
            first_name,
            last_name,
            email,
            birth_date,
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

    pub fn update(&mut self, updatable_properties: UserUpdatableProperties) {
        if let Some(username) = updatable_properties.username {
            self.username = username;
        }

        if let Some(first_name) = updatable_properties.first_name {
            self.first_name = first_name;
        }

        if let Some(last_name) = updatable_properties.last_name {
            self.last_name = last_name;
        }

        if let Some(email) = updatable_properties.email {
            self.email = email;
        }

        if let Some(birth_date) = updatable_properties.birth_date {
            self.birth_date = Some(birth_date);
        }

        if let Some(password_hash) = updatable_properties.password_hash {
            self.password_hash = password_hash;
        }

        if let Some(ask_for_new_password) = updatable_properties.ask_for_new_password {
            self.ask_for_new_password = ask_for_new_password;
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
            birth_date: self.birth_date.clone(),
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
            birth_date: self.birth_date,
            ask_for_new_password: self.ask_for_new_password,
        }
    }
}
