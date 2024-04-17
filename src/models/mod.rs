use serde::Deserialize;

pub mod account;
pub mod answer;
pub mod question;

#[derive(Debug, Deserialize, Default)]
pub struct Pagination {
    pub offset: Option<i64>,
    pub limit: Option<i64>,
}
