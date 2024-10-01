use crate::{
    core::domain::{entities::user::User, repository::user_repository::UserRepository},
    schema::user,
    shared::domain::repository::repository::Repository,
};

use super::user_model::UserModel;

pub struct UserDieselRepository {
    connection: diesel::PgConnection,
    user_model: UserModel,
    user_schema: user::table,
}

impl UserDieselRepository {
    pub fn new(
        connection: diesel::PgConnection,
        user_model: UserModel,
        user_schema: user::table,
    ) -> Self {
        UserDieselRepository {
            connection,
            user_model,
            user_schema,
        }
    }
}

impl Repository<User> for UserDieselRepository {
    fn find_all(&self) -> Vec<User> {
        let users = self
            .user_schema
            .load::<UserModel>(&self.connection)
            .unwrap();
        users
            .iter()
            .map(|user| User::from(user))
            .collect::<Vec<User>>()
    }
    fn find_by_id(&self, id: i32) -> Option<User> {
        let user = self
            .user_schema
            .find(id)
            .first::<UserModel>(&self.connection)
            .unwrap();

        Some(User::from(&user))
    }
    fn save(&self, user: User) -> Result<User, String> {
        let user_model = UserModel::from(&user);
        let result = diesel::insert_into(self.user_schema)
            .values(&user_model)
            .get_result(&self.connection)
            .unwrap();

        Ok(User::from(&result))
    }
    fn delete(&self, id: i32) -> Result<(), String> {
        let result = diesel::delete(self.user_schema.find(id)).execute(&self.connection);

        match result {
            Ok(_) => Ok(()),
            Err(_) => Err("Error deleting user".to_string()),
        }
    }
}

impl UserRepository for UserDieselRepository {}
