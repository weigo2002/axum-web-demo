use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Account {
    pub id: Option<AccountId>,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct AccountId(pub i32);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Session {
    pub exp: DateTime<Utc>,
    pub account_id: AccountId,
    pub nbf: DateTime<Utc>,
}
