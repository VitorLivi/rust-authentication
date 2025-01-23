use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize, Debug)]
pub struct PermissionProperties {
    pub id: Option<Uuid>,
    pub name: String,
}

#[derive(Serialize)]
pub struct Permission {
    id: Option<Uuid>,
    name: String,
}

pub struct PermissionUpdatableProperties {
    pub name: Option<String>,
}

impl Permission {
    pub fn new(id: Option<Uuid>, name: String) -> Permission {
        Permission {
            id: id.or(Some(Uuid::new_v4())),
            name,
        }
    }

    pub fn update(&mut self, updatable_properties: PermissionUpdatableProperties) -> &mut Self {
        if let Some(name) = updatable_properties.name {
            self.name = name;
        }

        self
    }

    pub fn get_properties(&self) -> PermissionProperties {
        PermissionProperties {
            id: self.id,
            name: self.name.clone(),
        }
    }
}
