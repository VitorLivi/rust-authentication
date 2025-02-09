use diesel::prelude::*;
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl, SelectableHelper,
};
use uuid::Uuid;

use crate::{
    core::domain::{
        entities::permission::Permission, repository::permission_repository::PermissionRepository,
    },
    schema::permission::{self},
    shared::domain::repository::repository::Repository,
};

use super::{permission_mapper::PermissionMapper, permission_model::PermissionModel};

pub struct PermissionDieselRepository {
    connection: PooledConnection<ConnectionManager<PgConnection>>,
    permission_schema: permission::table,
}

impl PermissionDieselRepository {
    pub fn new(
        connection: PooledConnection<ConnectionManager<PgConnection>>,
        permission_schema: permission::table,
    ) -> Self {
        PermissionDieselRepository {
            connection,
            permission_schema,
        }
    }
}

impl PermissionRepository for PermissionDieselRepository {}

impl Repository<Permission> for PermissionDieselRepository {
    fn find_all(&mut self) -> Vec<Permission> {
        let permissions = self
            .permission_schema
            .select(PermissionModel::as_select())
            .load::<PermissionModel>(&mut self.connection);

        match permissions {
            Ok(permissions) => permissions
                .iter()
                .map(|permission_model| PermissionMapper::to_entity(permission_model))
                .collect(),

            Err(_) => vec![],
        }
    }

    fn find_by_id(&mut self, id: Uuid) -> Result<Option<Permission>, None> {
        let permission_result = self
            .permission_schema
            .filter(permission::id.eq(id))
            .first::<PermissionModel>(&mut self.connection);

        match permission_result {
            Ok(permission_model) => Some(PermissionMapper::to_entity(&permission_model)),
            Err(_) => None,
        }
    }

    fn save(&mut self, permission: Permission) -> Result<Permission, String> {
        let result = diesel::insert_into(self.permission_schema)
            .values(PermissionMapper::to_model(&permission))
            .on_conflict(self.permission_schema.primary_key())
            .do_update()
            .set(PermissionMapper::to_model(&permission))
            .get_result::<PermissionModel>(&mut self.connection);

        match result {
            Ok(permission_model) => Ok(PermissionMapper::to_entity(&permission_model)),
            Err(err) => {
                println!("{:?}", err);
                Err("Error saving permission".to_string())
            }
        }
    }

    fn delete(&mut self, id: Uuid) -> Result<(), String> {
        let result = diesel::delete(self.permission_schema.filter(permission::id.eq(id)))
            .execute(&mut self.connection);

        match result {
            Ok(_) => Ok(()),
            Err(err) => {
                println!("{:?}", err);
                Err("Error deleting permission".to_string())
            }
        }
    }
}
