use axum::{routing::post, Router};

use crate::{handlers::answer::add_answer, repositories::store::Store};

pub fn create_router(store: Store) -> Router {
    Router::new()
        .route("/api/answers", post(add_answer))
        .with_state(store)
}
