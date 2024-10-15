use std::time::SystemTime;

use crate::schema::user;
use diesel::{pg::Pg, prelude::Queryable, Selectable};

#[derive(Queryable, Selectable)]
#[diesel(table_name = user, check_for_backend(Pg))]
pub struct UserModel {
    pub id: String,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
    pub first_name: String,
    pub last_name: String,
    pub username: Option<String>,
    pub email: String,
    pub password_hash: String,
    pub status: Option<i32>,
}
