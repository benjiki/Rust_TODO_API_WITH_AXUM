use axum::{extract::Path, http::StatusCode};

pub async fn handler(Path(id): Path<u64>) -> StatusCode {
    println!("todo delete {id}");
    StatusCode::OK
}
