use axum::{Json, http::StatusCode};

use crate::handlers::todo::models::Todo;



pub async fn handler()->(StatusCode,Json<Todo>){

    (
        StatusCode::OK,
        Json(Todo { id: 1, title: "something".to_string(), description: "desc of something".to_string() })
    )
}