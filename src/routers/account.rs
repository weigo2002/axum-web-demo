use axum::{routing::post, Router};

use crate::{
    handlers::account::{login, register},
    repositories::store::Store,
};

pub fn create_router(store: Store) -> Router {
    Router::new()
        .route("/api/registration", post(register))
        .route("/api/login", post(login))
        .with_state(store)
}
