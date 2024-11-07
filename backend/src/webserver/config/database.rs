use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use once_cell::sync::OnceCell;
use std::env;
use std::sync::Arc;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub struct Database {
    pool: OnceCell<Arc<Pool>>,
}

impl Database {
    pub fn init_pool() -> Pool {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.")
    }

    pub fn get_pool() -> Arc<Pool> {
        static POOL: OnceCell<Arc<Pool>> = OnceCell::new();
        POOL.get_or_init(|| Arc::new(Database::init_pool())).clone()
    }
}
