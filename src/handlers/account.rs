use argon2::Config;
use axum::{extract::State, Json};
use chrono::Utc;
use paseto::v2::local_paseto;
use rand::Rng;
use tracing::{event, Level};

use crate::{
    common::error::Error,
    models::account::{self, Account, AccountId},
    repositories::store::Store,
};

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

pub async fn login(
    State(store): State<Store>,
    Json(login): Json<Account>,
) -> Result<String, Error> {
    let token: Result<String, Error> = match store.get_account(login.email).await {
        Ok(account) => match verify_password(login.password.as_bytes(), &account.password) {
            Ok(verified) => {
                if verified {
                    Ok(issue_token(account.id.expect("id not found")))
                } else {
                    return Err(Error::WrongPassword);
                }
            }
            Err(e) => return Err(Error::ArgonLibraryError(e)),
        },
        Err(e) => return Err(e),
    };

    token
}

pub fn hash_passowrd(password: &[u8]) -> String {
    let salt = rand::thread_rng().gen::<[u8; 32]>();
    let config = Config::default();
    argon2::hash_encoded(password, &salt, &config).unwrap()
}

fn verify_password(password: &[u8], hash: &str) -> Result<bool, argon2::Error> {
    argon2::verify_encoded(hash, password)
}

fn issue_token(account_id: AccountId) -> String {
    let current_datetime = Utc::now();
    let dt = current_datetime + chrono::Duration::days(1);

    paseto::tokens::PasetoBuilder::new()
        .set_encryption_key(&Vec::from("RANDOM WORDS WINTER MACINTOSH PC".as_bytes()))
        .set_expiration(&dt)
        .set_not_before(&Utc::now())
        .set_claim("account_id", serde_json::json!(account_id))
        .build()
        .expect("Failed to construct paseto token w/ builder!")
}
