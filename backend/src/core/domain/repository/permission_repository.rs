use crate::{
    core::domain::entities::permission::Permission,
    shared::domain::repository::repository::Repository,
};

pub trait PermissionRepository: Repository<Permission> {}
