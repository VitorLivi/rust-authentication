use std::time::SystemTime;

use crate::schema::permission;

use diesel::prelude::*;
use diesel::{pg::Pg, Selectable};
use uuid::Uuid;

#[derive(Queryable, Selectable, QueryableByName, Insertable, AsChangeset)]
#[diesel(table_name = permission, check_for_backend(Pg))]
pub struct PermissionModel {
    pub id: Uuid,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
    pub name: String,
}
