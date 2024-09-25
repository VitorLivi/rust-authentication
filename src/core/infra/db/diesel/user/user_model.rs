use std::time::SystemTime;

use diesel::{prelude::Queryable, Selectable};

#[derive(Queryable, Selectable)]
#[diesel(table_name = user)]
pub struct UserModel {
    id: i32,
    created_at: SystemTime,
    updated_at: SystemTime,
    first_name: String,
    last_name: String,
    username: String,
    email: String,
    password_hash: String,
    status: i32,
}
