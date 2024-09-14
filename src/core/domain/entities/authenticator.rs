use std::io::Error;

use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};

use super::user::User;

pub struct Authenticator {}

impl Authenticator {
    pub fn new() -> Authenticator {
        Authenticator {}
    }

    pub fn authenticate(&self, user: &User) {
        let salt = SaltString::generate(&mut OsRng);
        let password = user.credentials.password.as_bytes();

        // TODO: Need to get hashed password from database
        let parsed_hash = PasswordHash::new(&password_hash)?;
        assert!(Argon2::default()
            .verify_password(password, &parsed_hash)
            .is_ok());
    }

    pub fn create_hash(&self, password: &str) -> String {
        let argon2 = Argon2::default();

        let password_hash = argon2
            .hash_password(password.as_bytes(), &SaltString::generate(&mut OsRng))
            .unwrap();

        return password_hash.to_string();
    }
}
