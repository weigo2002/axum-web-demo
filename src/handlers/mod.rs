use axum::{response::IntoResponse, Json};

pub mod account;
pub mod answer;
pub mod question;

pub async fn health_check_handler() -> impl IntoResponse {
    const MESSAGE: &str = "Server is running";

    let json_response = serde_json::json!({
        "status": "OK",
        "message": MESSAGE
    });

    Json(json_response)
}
