use axum::{
    routing::{delete, get, post, put},
    Router,
};

use crate::{
    handlers::question::{
        add_question, delete_question, get_question_byid, get_questions, update_question,
    },
    repositories::store::Store,
};

pub fn create_router(store: Store) -> Router {
    Router::new()
        .route("/api/questions", post(add_question))
        .route("/api/questions", get(get_questions))
        .route("/api/questions/:id", get(get_question_byid))
        .route("/api/questions/:id", put(update_question))
        .route("/api/questions/:id", delete(delete_question))
        .with_state(store)
}
