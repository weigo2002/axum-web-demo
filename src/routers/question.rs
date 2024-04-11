use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    handlers::question::{add_question, get_question_byid, get_questions},
    repositories::store::Store,
};

pub fn create_router(store: Store) -> Router {
    Router::new()
        .route("/api/questions", post(add_question))
        .route("/api/questions", get(get_questions))
        .route("/api/questions/:id", get(get_question_byid))
        .with_state(store)
}
