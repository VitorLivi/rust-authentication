use crate::{
    core::domain::entities::group::Group, shared::domain::repository::repository::Repository,
};

pub trait GroupRepository: Repository<Group> {
}
