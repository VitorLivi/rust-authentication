use actix_session::Session;

use crate::core::domain::entities::authenticator::Authenticator;
use crate::core::domain::entities::user::{User, UserCredentials};
use crate::shared::use_cases::use_case::UseCase;

pub struct AuthenticateUserInputDto {
    username: String,
    password: String,
    session: Session,
}

impl AuthenticateUserInputDto {
    pub fn new(username: String, password: String, session: Session) -> AuthenticateUserInputDto {
        AuthenticateUserInputDto {
            username,
            password,
            session,
        }
    }
}

pub struct AuthenticateUser {}

impl UseCase<AuthenticateUserInputDto, ()> for AuthenticateUser {
    fn new() -> AuthenticateUser {
        AuthenticateUser {}
    }

    fn execute(&self, input: AuthenticateUserInputDto) -> () {
        let authenticator = Authenticator::new();

        let user_credentials = UserCredentials::new(input.username, input.password);
        let user = User::new(user_credentials);

        user.authenticate(authenticator);
    }
}
