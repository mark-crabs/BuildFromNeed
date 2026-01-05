use crate::{
    db::{DbPool, establish_db_connection},
    files::{load_env, types::ENV},
};

pub struct AppState {
    pub db_pool: DbPool,
    pub env: ENV,
}

impl AppState {
    pub fn new() -> Self {
        let env: ENV =
            load_env().unwrap_or_else(|_| panic!("Failed to access .env. Check it if exists."));
        Self {
            db_pool: establish_db_connection(&env.database_url),
            env,
        }
    }
}
