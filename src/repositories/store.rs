use sqlx::{
    postgres::{PgPoolOptions, PgRow},
    PgPool, Row,
};

use crate::{
    common::error::Error,
    models::question::{NewQuestion, Question, QuestionId},
};

#[derive(Debug, Clone)]
pub struct Store {
    pub connection: PgPool,
}

impl Store {
    pub async fn new(db_url: &str) -> Self {
        let db_pool = match PgPoolOptions::new()
            .max_connections(5)
            .connect(db_url)
            .await
        {
            Ok(pool) => pool,
            Err(e) => panic!("Couldn't establish database connection: {}", e),
        };

        Store {
            connection: db_pool,
        }
    }

    pub async fn add_question(&self, new_question: NewQuestion) -> Result<Question, Error> {
        match sqlx::query(
            "INSERT INTO questions (title, content, tags) VALUES ($1, $2, $3)
                 RETURNING *",
        )
        .bind(new_question.title)
        .bind(new_question.content)
        .bind(new_question.tags)
        .map(|row: PgRow| Question {
            id: QuestionId(row.get("id")),
            title: row.get("title"),
            content: row.get("content"),
            tags: row.get("tags"),
        })
        .fetch_one(&self.connection)
        .await
        {
            Ok(question) => Ok(question),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(Error::DatabaseQueryError)
            }
        }
    }
}
