use argon2::Config;
use axum::{extract::State, Json};
use rand::Rng;
use tracing::{event, Level};

use crate::{common::error::Error, models::account::Account, repositories::store::Store};

pub async fn register(
    State(store): State<Store>,
    Json(account): Json<Account>,
) -> Result<String, Error> {
    event!(target:"axum-web-dev", Level::INFO, "register new user");
    let hashed_pwd = hash_passowrd(account.password.as_bytes());

    let account = Account {
        id: account.id,
        email: account.email,
        password: hashed_pwd,
    };

    let _res: Result<bool, Error> = match store.add_account(account).await {
        Err(e) => return Err(e),
        Ok(res) => Ok(res),
    };

    Ok(String::from("Success"))
}

pub fn hash_passowrd(password: &[u8]) -> String {
    let salt = rand::thread_rng().gen::<[u8; 32]>();
    let config = Config::default();
    argon2::hash_encoded(password, &salt, &config).unwrap()
}
