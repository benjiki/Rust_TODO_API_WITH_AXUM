use axum::{Json, http::StatusCode};

use crate::handlers::todo::models::Todo;

pub async fn handler() -> (StatusCode, Json<Vec<Todo>>) {
    (
        StatusCode::OK,
        Json(vec![Todo {
            id: 1,
            title: "Some title".to_string(),
            description: "some desc".to_string(),
        }]),
    )
}
