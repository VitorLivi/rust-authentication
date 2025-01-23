use crate::core::domain::entities::permission::Permission;
use crate::core::infra::db::diesel::permission::permission_model::PermissionModel;
use std::time::SystemTime;

pub struct PermissionMapper {}

impl PermissionMapper {
    pub fn to_entity(permission_model: &PermissionModel) -> Permission {
        Permission::new(
            Some(permission_model.id),
            permission_model.name.clone()
        )
    }

    pub fn to_model(permission: &Permission) -> PermissionModel {
        let permission_props = permission.get_properties();

        PermissionModel {
            id: permission_props.id.unwrap(),
            name: permission_props.name,
            created_at: SystemTime::now(),
            updated_at: SystemTime::now(),
        }
    }
}
