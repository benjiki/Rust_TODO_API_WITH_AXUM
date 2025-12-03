use axum::{Json, http::StatusCode};

use crate::handlers::todo::models::CreateTodoRequest;

pub async fn handler(
    Json(request): Json<CreateTodoRequest>,
) -> (StatusCode, Json<CreateTodoRequest>) {
    println!("Create TODO {request:?} ");

    (
        StatusCode::OK,
        Json(CreateTodoRequest {
            title: "somethings".to_string(),
            description: "desc".to_string(),
        }),
    )
}
