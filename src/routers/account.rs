use axum::{routing::post, Router};

use crate::{handlers::account::register, repositories::store::Store};

pub fn create_router(store: Store) -> Router {
    Router::new()
        .route("/api/registration", post(register))
        .with_state(store)
}
