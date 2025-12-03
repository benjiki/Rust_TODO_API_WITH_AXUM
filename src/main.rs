use axum::{Router, routing::get};
mod handlers;

#[tokio::main]
async  fn main()->anyhow::Result<()> {

let router=Router::new().route("/list",get(handlers::todo::list::handler) );
let listener=tokio::net::TcpListener::bind("0.0.0.0:9999").await?;
axum::serve(listener,router).await?;
Ok(())
}
