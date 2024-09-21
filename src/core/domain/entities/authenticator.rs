use std::{borrow::Borrow, io::Error};

use actix_session::Session;
use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};

use super::user::User;

pub struct Authenticator {
    session: Session,
}

impl Authenticator {
    pub fn new(session: Session) -> Authenticator {
        Authenticator { session }
    }

    pub fn authenticate(&mut self, user: &User) -> bool {
        let is_password_correct = self.verify_password(user);

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
            // print error
            Err(e) => {
                println!("Error: {}", e);
                return false;
            }
        }

        return true;
    }

    pub fn verify_password(&self, user: &User) -> bool {
        let argon2 = Argon2::default();

        let password_hash = PasswordHash::new(&user.password_hash).unwrap();

        return argon2
            .verify_password(
                user.credentials.as_ref().unwrap().password.as_bytes(),
                &password_hash,
            )
            .is_ok();
    }

    pub fn create_hash(password: &str) -> String {
        let argon2 = Argon2::default();
        let salt = SaltString::generate(&mut OsRng);

        println!("Salt: {}", salt);

        let password_hash = argon2.hash_password(password.as_bytes(), &salt).unwrap();

        return password_hash.to_string();
    }
}
