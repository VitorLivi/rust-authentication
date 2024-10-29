use uuid::Uuid;

use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl, SelectableHelper,
};

use crate::{
    core::domain::{entities::user::User, repository::user_repository::UserRepository},
    schema::user::{self},
    shared::domain::repository::repository::Repository,
};

use super::{user_mapper::UserMapper, user_model::UserModel};

pub struct UserDieselRepository {
    connection: PooledConnection<ConnectionManager<PgConnection>>,
    user_schema: user::table,
}

impl UserDieselRepository {
    pub fn new(
        connection: PooledConnection<ConnectionManager<PgConnection>>,
        user_schema: user::table,
    ) -> Self {
        UserDieselRepository {
            connection,
            user_schema,
        }
    }
}

impl UserRepository for UserDieselRepository {
    fn find_by_username(&mut self, username: String) -> Option<User> {
        let user_result = self
            .user_schema
            .filter(user::username.eq(username))
            .first::<UserModel>(&mut self.connection);

        match user_result {
            Ok(user_model) => Some(UserMapper::to_entity(&user_model)),
            Err(e) => {
                println!("{:?}", e);
                None
            }
        }
    }
}

impl Repository<User> for UserDieselRepository {
    fn find_all(&mut self) -> Vec<User> {
        let users = self
            .user_schema
            .select(UserModel::as_select())
            .load::<UserModel>(&mut self.connection);

        match users {
            Ok(users) => users
                .iter()
                .map(|user_model| UserMapper::to_entity(user_model))
                .collect(),

            Err(_) => vec![],
        }
    }

    fn find_by_id(&mut self, id: Uuid) -> Option<User> {
        let user_result = self
            .user_schema
            .filter(user::id.eq(id))
            .first::<UserModel>(&mut self.connection);

        match user_result {
            Ok(user_model) => Some(UserMapper::to_entity(&user_model)),
            Err(_) => None,
        }
    }

    fn save(&mut self, user: User) -> Result<User, String> {
        let result = diesel::insert_into(self.user_schema)
            .values(UserMapper::to_model(&user))
            .get_result::<UserModel>(&mut self.connection);

        if result.is_err() {
            println!("{:?}", result.err());
            return Err("Error saving user".to_string());
        }

        return Ok(UserMapper::to_entity(&result.unwrap()));
    }

    fn delete(&mut self, id: Uuid) -> Result<(), String> {
        let result = diesel::update(self.user_schema.filter(user::id.eq(id)))
            .set(user::status.eq(0))
            .execute(&mut self.connection);

        match result {
            Ok(_) => Ok(()),
            Err(_) => Err("Error deleting user".to_string()),
        }
    }
}
