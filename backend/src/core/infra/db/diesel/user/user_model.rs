use std::time::SystemTime;

use crate::schema::user;
use chrono::NaiveDate;
use diesel::prelude::*;
use diesel::{pg::Pg, Selectable};
use uuid::Uuid;

#[derive(Queryable, Selectable, QueryableByName, Insertable, AsChangeset)]
#[diesel(table_name = user, check_for_backend(Pg))]
pub struct UserModel {
    pub id: Uuid,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
    pub first_name: String,
    pub last_name: String,
    pub username: Option<String>,
    pub email: String,
    pub password_hash: String,
    pub status: Option<i32>,
    pub ask_for_new_password: i32,
    pub birth_date: Option<NaiveDate>,
}
