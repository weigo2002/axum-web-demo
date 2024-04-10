use axum::{
    routing::{get, post},
    Router,
};
use tower_http::trace::TraceLayer;

use crate::{
    handlers::{health_check_handler, question::add_question},
    repositories::store::Store,
};

pub mod question;

pub fn create_router(store: Store) -> Router {
    Router::new()
        .route("/api/healthcheck", get(health_check_handler))
        .route("/questions", post(add_question))
        .layer(TraceLayer::new_for_http())
        .with_state(store)
}
