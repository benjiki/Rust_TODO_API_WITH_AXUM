use axum::{
    Router,
    routing::{delete, get, patch, post, put},
};

use crate::db::{connection_pool, migration::migrate_db};

mod handlers;

mod db;

#[tokio::main]

async fn main() -> anyhow::Result<()> {
    migrate_db()?;
    let db_conn_pool = connection_pool()?;

    let router = Router::new()
        .route("/todo", get(handlers::todo::list::handler))
        .route("/todo/{id}", delete(handlers::todo::delete::handler))
        .route("/todo/{id}", get(handlers::todo::get::handler))
        .route("/todo", post(handlers::todo::create::handler))
        .route("/todo/{id}", patch(handlers::todo::partial_update::handler))
        .route("/todo/{id}", put(handlers::todo::list::handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:9999").await?;
    axum::serve(listener, router).await?;

    Ok(())
}
