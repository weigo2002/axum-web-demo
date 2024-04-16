use axum::{extract::State, Json};
use tracing::{event, instrument, Level};

use crate::{
    common::error::Error,
    models::answer::{Answer, NewAnswer},
    repositories::store::Store,
};

#[instrument]
pub async fn add_answer(
    State(store): State<Store>,
    Json(new_answer): Json<NewAnswer>,
) -> Result<Json<Answer>, Error> {
    event!(target:"axum-web-dev", Level::INFO, "add new answer");
    let res = match store.add_answer(new_answer).await {
        Err(e) => return Err(e),
        Ok(res) => res,
    };

    Ok(Json(res))
}
