pub mod schema;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub fn establish_db_connection(database_url: &str) -> DbPool {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .max_size(15)
        .build(manager)
        .expect("Failed to create DB pool")
}
