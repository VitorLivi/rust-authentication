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
    pub credentials: UserCredentials,
}

impl User {
    pub fn new(credentials: UserCredentials) -> User {
        User { credentials }
    }

    pub fn authenticate(&self, authenticator: Authenticator) {
        authenticator.authenticate(self);
    }
}
