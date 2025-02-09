use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize, Debug)]
pub struct GroupProperties {
    pub id: Option<Uuid>,
    pub name: String,
}

#[derive(Serialize)]
pub struct Group {
    id: Option<Uuid>,
    name: String,
}

pub struct GroupUpdatableProperties {
    pub name: Option<String>,
}

impl Group {
    pub fn new(id: Option<Uuid>, name: String) -> Group {
        Group {
            id: id.or(Some(Uuid::new_v4())),
            name,
        }
    }

    pub fn update(&mut self, updatable_properties: GroupUpdatableProperties) -> &mut Self {
        if let Some(name) = updatable_properties.name {
            self.name = name;
        }

        self
    }

    pub fn get_properties(&self) -> GroupProperties {
        GroupProperties {
            id: self.id,
            name: self.name.clone(),
        }
    }
}
