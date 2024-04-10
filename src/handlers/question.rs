use axum::{extract::State, Json};
use tracing::{event, instrument, Level};

use crate::{
    common::error::Error,
    models::question::{NewQuestion, Question},
    repositories::store::Store,
};

#[instrument]
pub async fn add_question(
    State(store): State<Store>,
    Json(new_question): Json<NewQuestion>,
) -> Result<Json<Question>, Error> {
    event!(target:"axum-web-dev", Level::INFO, "add new question");
    let res = match store.add_question(new_question).await {
        Err(e) => return Err(e),
        Ok(res) => res,
    };

    Ok(Json(res))
}
