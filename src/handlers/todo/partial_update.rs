use axum::{Json, http::StatusCode};

use crate::handlers::todo::models::PartialUpdateTodoRequest;

pub async fn handler(
    Json(request): Json<PartialUpdateTodoRequest>,
) -> (StatusCode, Json<PartialUpdateTodoRequest>) {
    println!("Partial Update Todo {request:?} ");

    (
        StatusCode::OK,
        Json(PartialUpdateTodoRequest {
            title: Some("somethings".to_string()),
            description: Some("desc".to_string()),
        }),
    )
}
