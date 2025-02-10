use diesel::prelude::*;
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl, SelectableHelper,
};
use uuid::Uuid;

use super::{group_mapper::GroupMapper, group_model::GroupModel};
use crate::{
    core::domain::{entities::group::Group, repository::group_repository::GroupRepository},
    schema::group::{self},
    shared::domain::repository::repository::Repository,
    shared::webserver::errors::webservice_error::WebserviceError,
};
use actix_web::{Result};

pub struct GroupDieselRepository {
    connection: PooledConnection<ConnectionManager<PgConnection>>,
    group_schema: group::table,
}

impl GroupDieselRepository {
    pub fn new(
        connection: PooledConnection<ConnectionManager<PgConnection>>,
        group_schema: group::table,
    ) -> Self {
        GroupDieselRepository {
            connection,
            group_schema,
        }
    }
}

impl GroupRepository for GroupDieselRepository {}

impl Repository<Group> for GroupDieselRepository {
    fn find_all(&mut self) -> Vec<Group> {
        let groups = self
            .group_schema
            .select(GroupModel::as_select())
            .load::<GroupModel>(&mut self.connection);

        match groups {
            Ok(groups) => groups
                .iter()
                .map(|group_model| GroupMapper::to_entity(group_model))
                .collect(),

            Err(_) => vec![],
        }
    }

    fn find_by_id(&mut self, id: Uuid) -> Result<Option<Group>, String> {
        let group_result = self
            .group_schema
            .filter(group::id.eq(id))
            .first::<GroupModel>(&mut self.connection);

        match group_result {
            Ok(group_model) => Ok(Some(GroupMapper::to_entity(&group_model))),
            Err(_) => Err("Error finding group".to_string()),
        }
    }

    fn save(&mut self, group: Group) -> Result<Group, String> {
        let result = diesel::insert_into(self.group_schema)
            .values(GroupMapper::to_model(&group))
            .on_conflict(self.group_schema.primary_key())
            .do_update()
            .set(GroupMapper::to_model(&group))
            .get_result::<GroupModel>(&mut self.connection);

        match result {
            Ok(group_model) => Ok(GroupMapper::to_entity(&group_model)),
            Err(err) => {
                println!("{:?}", err);
                Err("Error saving group".to_string())
            }
        }
    }

    fn delete(&mut self, id: Uuid) -> Result<(), String> {
        let result = diesel::delete(self.group_schema.filter(group::id.eq(id)))
            .execute(&mut self.connection);

        match result {
            Ok(_) => Ok(()),
            Err(err) => {
                println!("{:?}", err);
                Err("Error deleting group".to_string())
            }
        }
    }
}
