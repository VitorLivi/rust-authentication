use uuid::Uuid;

use crate::core::domain::entities::authenticator::Authenticator;
use crate::core::domain::entities::user::User;
use crate::shared::use_cases::use_case::UseCase;

pub struct CreateUserInputDto {
    username: String,
    password: String,
}

impl CreateUserInputDto {
    pub fn new(username: String, password: String) -> CreateUserInputDto {
        CreateUserInputDto { username, password }
    }
}

pub struct CreateUser {}

impl UseCase<CreateUserInputDto, ()> for CreateUser {
    fn new() -> CreateUser {
        CreateUser {}
    }

    fn execute(&self, input: CreateUserInputDto) -> () {
        print!("CREATING USER");

        let uuid = Uuid::new_v4();

        let password_hash = Authenticator::create_hash(&input.password);

        println!("User created with id: {}", password_hash);
        let user = User::new(Some(uuid), None, password_hash);

        // store user in database
    }
}
