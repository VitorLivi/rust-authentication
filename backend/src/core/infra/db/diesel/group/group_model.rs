use std::time::SystemTime;

use crate::schema::user;
use chrono::NaiveDate;
use diesel::prelude::*;
use diesel::{pg::Pg, Selectable};
use uuid::Uuid;

#[derive(Queryable, Selectable, QueryableByName, Insertable, AsChangeset)]
#[diesel(table_name = group, check_for_backend(Pg))]
pub struct GroupModel {
    pub id: Uuid,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
    pub name: String,
}
