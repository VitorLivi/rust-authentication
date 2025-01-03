use std::time::SystemTime;

use crate::schema::user;
use diesel::prelude::*;
use diesel::{pg::Pg, Selectable};
use uuid::Uuid;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = user, check_for_backend(Pg))]
pub struct UserModel {
    pub id: Uuid,
    pub created_at: Option<SystemTime>,
    pub updated_at: Option<SystemTime>,
    pub first_name: String,
    pub last_name: String,
    pub username: Option<String>,
    pub email: String,
    pub password_hash: String,
    pub status: Option<i32>,
    pub ask_reset_password: i32,
}
