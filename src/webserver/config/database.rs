use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::Connection;
use std::env;
use std::sync::{Mutex, Once};

pub struct Database {
    pub connection_pool: Pool<ConnectionManager<PgConnection>>,
}

impl Database {
    fn config() -> PgConnection {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        PgConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
    }

    pub fn get_connection() -> &PgConnection {
        static mut CONN: Option<PgConnection> = None;
        static INIT: Once = Once::new();
        unsafe {
            INIT.call_once(|| {
                CONN = Some(Database::config());
            });

            return match CONN {
                Some(ref conn) => conn,
                None => panic!("Failed to initialize connection"),
            };
        }
    }
}
