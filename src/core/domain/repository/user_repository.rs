use crate::{
    core::domain::entities::user::User, shared::domain::repository::repository::Repository,
};

pub trait UserRepository: Repository<User> {}
