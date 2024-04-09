use axum::{routing::get, Router};

use crate::handlers::health_check_handler;

pub fn create_router() -> Router {
    Router::new().route("/api/healthcheck", get(health_check_handler))
}
