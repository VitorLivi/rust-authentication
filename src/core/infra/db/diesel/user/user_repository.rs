use std::borrow::{Borrow, BorrowMut};

use diesel::{
    dsl::select,
    query_dsl::methods::LoadQuery,
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection, QueryDsl, Queryable, RunQueryDsl, SelectableHelper, Table,
};

use crate::{
    core::domain::{entities::user::User, repository::user_repository::UserRepository},
    schema::user::{self, all_columns, table},
    shared::domain::repository::repository::Repository,
};

use super::{user_mapper::UserMapper, user_model::UserModel};

pub struct UserDieselRepository {
    connection: PgConnection,
    user_schema: user::table,
}

impl UserDieselRepository {
    pub fn new(connection: PgConnection, user_schema: user::table) -> Self {
        UserDieselRepository {
            connection,
            user_schema,
        }
    }
}

impl UserRepository for UserDieselRepository {}

impl Repository<User> for UserDieselRepository {
    fn find_all(&mut self) -> Vec<User> {
        let mutable_connection = self.connection.borrow_mut();
        let users = self
            .user_schema
            .select(UserModel::as_select())
            .load::<UserModel>(mutable_connection);

        match users {
            Ok(users) => users
                .iter()
                .map(|user_model| UserMapper::to_entity(user_model))
                .collect(),

            Err(_) => vec![],
        }
    }

    fn find_by_id(&self, id: i32) -> Option<User> {}

    fn save(&self, user: User) -> Result<User, String> {}

    fn delete(&self, id: i32) -> Result<(), String> {}
}
