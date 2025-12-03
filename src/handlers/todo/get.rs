use axum::{Json, extract::Path, http::StatusCode};

use crate::handlers::todo::models::Todo;

pub async fn handler(Path(id): Path<u64>) -> (StatusCode, Json<Todo>) {
    println!("user id {id}");
    (
        StatusCode::OK,
        Json(Todo {
            id: { id },
            title: "something".to_string(),
            description: "desc of something".to_string(),
        }),
    )
}
