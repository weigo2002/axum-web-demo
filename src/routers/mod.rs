use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use tower_http::trace::TraceLayer;

use crate::{
    handlers::{account::auth, health_check_handler},
    repositories::store::Store,
};

pub mod account;
pub mod answer;
pub mod question;

pub fn create_router(store: Store) -> Router {
    Router::new()
        .route("/api/healthcheck", get(health_check_handler))
        .merge(question::create_router(store.clone()))
        .merge(answer::create_router(store.clone()))
        .merge(account::create_router(store.clone()))
        .layer(middleware::from_fn(auth))
        .layer(TraceLayer::new_for_http())
}
