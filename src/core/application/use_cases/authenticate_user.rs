use actix_session::Session;
use uuid::Uuid;

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
        let mut authenticator = Authenticator::new(input.session);
        let user_credentials = UserCredentials::new(input.username, input.password);

        let test_id = Uuid::new_v4();

        let mut user = User::new(Some(test_id), Some(user_credentials), "$argon2id$v=19$m=19456,t=2,p=1$IYfzwMcQ8cphz7Y8+WFtyg$Hc4MuXJdNYpEl/hunDxVUzSqbfgJXPu+0OdBijlAUI4".to_string());

        user.authenticate(&mut authenticator);
    }
}
