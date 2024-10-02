// create a singleton to hold the database connection

use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use std::env;

pub struct Database {
    connection_pool: Option<&'static Pool<ConnectionManager<PgConnection>>>,
}

impl Database {
    pub fn config() -> Database {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let connection_pool = Pool::builder()
            .build(manager)
            .expect("Failed to create pool");

        Self::connection_pool = Some(connection_pool);
    }

    pub fn get_connection() -> PgConnection {
        if !Self.connection_pool {
            let me = Database::config();
            Self.connection_pool = Some(&me.connection_pool);
        }

        Database::conection_pool
    }
}
