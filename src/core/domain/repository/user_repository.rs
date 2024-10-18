use crate::{
    core::domain::entities::user::User, shared::domain::repository::repository::Repository,
};

pub trait UserRepository: Repository<User> {
    fn find_by_username(&self, username: String) -> Option<User>;
}
