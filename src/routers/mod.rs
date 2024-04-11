use axum::{
    routing::{get, post},
    Router,
};
use tower_http::trace::TraceLayer;

use crate::{handlers::health_check_handler, repositories::store::Store};

pub mod question;

pub fn create_router(store: Store) -> Router {
    Router::new()
        .route("/api/healthcheck", get(health_check_handler))
        .merge(question::create_router(store))
        .layer(TraceLayer::new_for_http())
}
