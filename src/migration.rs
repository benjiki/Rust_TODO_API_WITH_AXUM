use std::env;

use anyhow::Result;
use anyhow::anyhow;
// Correct import
use diesel::PgConnection;
use diesel::prelude::*;
use diesel_migrations::MigrationHarness;
use diesel_migrations::{EmbeddedMigrations, embed_migrations};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

pub fn migrate_db() -> anyhow::Result<()> {
    let mut connection = db_connection()?;
    connection
        .run_pending_migrations(MIGRATIONS)
        .map_err(|error| anyhow!("failed to run migration: {error}"))?;
    Ok(())
}

fn db_connection() -> Result<PgConnection> {
    // Load .env file correctly
    dotenvy::dotenv().ok();

    let database_url = env::var("DATABASE_URL")?;
    let connection = PgConnection::establish(&database_url)?;
    Ok(connection)
}
