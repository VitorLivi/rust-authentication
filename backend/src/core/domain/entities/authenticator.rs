use actix_session::Session;
use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};

use super::user::{User, UserCredentials};

pub struct Authenticator<'a> {
    session: &'a Session,
}

impl<'a> Authenticator<'a> {
    pub fn new(session: &'a Session) -> Authenticator {
        Authenticator { session }
    }

    pub fn authenticate(&mut self, user: &User, user_credentials: &UserCredentials) -> bool {
        let is_password_correct = self.verify_password(user, user_credentials);

        if !is_password_correct {
            self.session.clear();
            return false;
        }

        let user_id = user.get_id();

        println!("User id: {:?}", user_id);

        if user_id.is_none() {
            self.session.clear();
            return false;
        }

        let result = self.session.insert("USER_ID", user_id.unwrap().to_string());

        match result {
            Ok(_) => (),
            Err(e) => {
                println!("Error: {}", e);
                return false;
            }
        }

        return true;
    }

    pub fn verify_password(&self, user: &User, user_credentials: &UserCredentials) -> bool {
        let argon2 = Argon2::default();

        let user_properties = user.get_properties();
        let password_hash_result = PasswordHash::new(&user_properties.password_hash);

        match password_hash_result {
            Ok(password_hash) => argon2
                .verify_password(user_credentials.password.as_bytes(), &password_hash)
                .is_ok(),
            Err(_) => {
                return false;
            }
        }
    }

    pub fn create_hash(password: &str) -> String {
        let argon2 = Argon2::default();
        let salt = SaltString::generate(&mut OsRng);

        println!("Salt: {}", salt);

        let password_hash = argon2.hash_password(password.as_bytes(), &salt).unwrap();

        return password_hash.to_string();
    }
}
