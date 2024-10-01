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
    pub credentials: Option<UserCredentials>,
    pub password_hash: String,
    pub is_authenticated: bool,
}

impl User {
    pub fn new(
        id: Option<Uuid>,
        credentials: Option<UserCredentials>,
        password_hash: String,
    ) -> User {
        User {
            id: id.or(Some(Uuid::new_v4())),
            credentials,
            password_hash,
            is_authenticated: false,
        }
    }

    pub fn get_id(&self) -> Option<Uuid> {
        return self.id;
    }

    pub fn authenticate(&mut self, authenticator: &mut Authenticator) {
        print!("Authenticating user");

        let is_authenticated = authenticator.authenticate(&self);

        if is_authenticated {
            self.is_authenticated = true;
        }

        return;
    }
}
