use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    Json,
};
use tracing::{event, instrument, Level};

use crate::{
    common::error::Error,
    models::{
        question::{NewQuestion, Question},
        Pagination,
    },
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

#[instrument]
pub async fn get_questions(
    State(store): State<Store>,
    pagination: Option<Query<Pagination>>,
) -> Result<Json<Vec<Question>>, Error> {
    event!(target:"axum-web-demo", Level::INFO, "get pagination questions");
    let Query(pagination) = pagination.unwrap_or_default();
    let offset: i64 = pagination.offset.unwrap_or(0);
    let limit: i64 = pagination.limit.unwrap_or(100);
    let res: Vec<Question> = match store.get_questions(offset, limit).await {
        Ok(res) => res,
        Err(e) => return Err(e),
    };

    Ok(Json(res))
}

#[instrument]
pub async fn get_question_byid(
    State(store): State<Store>,
    Path(id): Path<i64>,
) -> Result<Json<Question>, Error> {
    event!(target:"axum-web-demo", Level::INFO, "get question by id");
    let res = match store.get_question_byid(id).await {
        Ok(res) => res,
        Err(e) => return Err(e),
    };

    Ok(Json(res))
}

#[instrument]
pub async fn update_question(
    State(store): State<Store>,
    Path(id): Path<i64>,
    Json(question): Json<Question>,
) -> Result<Json<Question>, Error> {
    event!(target:"axum-web-dev", Level::INFO, "update question");
    let res = match store.update_question(question, id).await {
        Err(e) => return Err(e),
        Ok(res) => res,
    };

    Ok(Json(res))
}

#[instrument]
pub async fn delete_question(
    State(store): State<Store>,
    Path(id): Path<i64>,
) -> Result<String, Error> {
    event!(target:"axum-web-dev", Level::INFO, "delete question");
    if let Err(e) = store.delete_question(id).await {
        return Err(e);
    }

    Ok(String::from("Question Deleted"))
}
