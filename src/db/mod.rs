use std::env;

use diesel::{
    PgConnection,
    r2d2::{ConnectionManager, Pool},
};
use dotenvy::dotenv;

pub mod migration;
pub mod models;
pub mod schema;

// getting a connection pool for database
pub fn connection_pool() -> anyhow::Result<Pool<ConnectionManager<PgConnection>>> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")?;
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Ok(Pool::builder().test_on_check_out(true).build(manager)?)
}
