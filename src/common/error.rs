use std::fmt::write;

use argon2::Error as ArgonError;
use axum::{response::IntoResponse, Json};
use reqwest::{Error as ReqwestError, StatusCode};
use serde_json::json;

#[derive(Debug)]
pub enum Error {
    ParseError(std::num::ParseIntError),
    MissingParameters,
    QuestionNotFound,
    DatabaseQueryError(sqlx::Error),
    ExternalAPIError(ReqwestError),
    WrongPassword,
    Unahthorized,
    ArgonLibraryError(ArgonError),
    CannotDecryptToken,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            Error::ParseError(ref err) => {
                write!(f, "Cannot parse parameter: {}", err)
            }
            Error::MissingParameters => write!(f, "Missing parameters"),
            Error::WrongPassword => write!(f, "Wrong password"),
            Error::ArgonLibraryError(_) => write!(f, "Cannot verify password"),
            Error::QuestionNotFound => write!(f, "Question not found"),
            Error::DatabaseQueryError(_) => write!(f, "Cannot update, invalid data."),
            Error::ExternalAPIError(err) => {
                write!(f, "Cannot execute: {}", err)
            }
            Error::CannotDecryptToken => write!(f, "Invalid token"),
            Error::Unahthorized => write!(f, "No resource permission"),
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let (status, err_msg) = match self {
            Self::DatabaseQueryError(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "internal server error")
            }
            Self::MissingParameters => (StatusCode::BAD_REQUEST, "Missing parameters"),
            Self::WrongPassword => (StatusCode::UNAUTHORIZED, "Invalid user name or password"),
            Self::ArgonLibraryError(_) => {
                (StatusCode::UNAUTHORIZED, "Invalid user name or password")
            }

            Self::QuestionNotFound => (StatusCode::BAD_REQUEST, "Question not found"),
            Self::ExternalAPIError(_err) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "External API call error")
            }
            Self::ParseError(ref err) => {
                //let e_msg = format!("Cannot parse parameter: {}", err);
                (StatusCode::BAD_REQUEST, "Parse parameter error")
            }
            Self::CannotDecryptToken => (StatusCode::UNAUTHORIZED, "Invalid token"),
            Self::Unahthorized => (StatusCode::UNAUTHORIZED, "No resource permission"),
        };
        (status, Json(json!({"error": err_msg}))).into_response()
    }
}
