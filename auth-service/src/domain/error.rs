use crate::domain::data_stores::{TwoFACodeStoreError, UserStoreError};
use crate::domain::email::ParseError;
use crate::domain::password::PasswordError;
use crate::ErrorResponse;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use color_eyre::eyre::eyre;
use color_eyre::Report;
use std::error::Error;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthAPIError {
    #[error("User already exists")]
    UserAlreadyExists,
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("Incorrect credentials")]
    IncorrectCredentials,
    #[error("Missing token")]
    MissingToken,
    #[error("Invalid token")]
    InvalidToken,
    #[error("Unexpected error")]
    UnexpectedError(#[source] Report),
    #[error("InvalidEmail")]
    InvalidEmail,
    #[error("MalformedRequest")]
    MalformedRequest,
    #[error("InvalidLoginId")]
    InvalidLoginId,
}
impl IntoResponse for AuthAPIError {
    fn into_response(self) -> Response {
        log_error_chain(&self);
        let (status, error_message) = match self {
            AuthAPIError::UserAlreadyExists => (StatusCode::CONFLICT, "User already exists"),
            AuthAPIError::InvalidCredentials => (StatusCode::BAD_REQUEST, "Invalid credentials"),
            AuthAPIError::InvalidLoginId => (StatusCode::BAD_REQUEST, "Invalid credentials"),
            AuthAPIError::MalformedRequest => {
                (StatusCode::UNPROCESSABLE_ENTITY, "Malformed Request")
            }

            AuthAPIError::UnexpectedError(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Unexpected error")
            }
            AuthAPIError::InvalidEmail => (StatusCode::BAD_REQUEST, "Invalid email"),
            AuthAPIError::IncorrectCredentials => (StatusCode::UNAUTHORIZED, "Wrong password"),
            AuthAPIError::MissingToken => (StatusCode::BAD_REQUEST, "Missing JWT Token"),
            AuthAPIError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid JWT Token"),
        };
        let body = Json(ErrorResponse {
            error: error_message.to_string(),
        });
        (status, body).into_response()
    }
}

fn log_error_chain(e: &(dyn Error + 'static)) {
    let separator =
        "\n-----------------------------------------------------------------------------------\n";
    let mut report = format!("{}{:?}\n", separator, e);
    let mut current = e.source();
    while let Some(cause) = current {
        let str = format!("Caused by:\n\n{:?}", cause);
        report = format!("{}\n{}", report, str);
        current = cause.source();
    }
    report = format!("{}\n{}", report, separator);
    tracing::error!("{}", report);
}
impl From<uuid::Error> for AuthAPIError {
    fn from(_error: uuid::Error) -> Self {
        AuthAPIError::InvalidLoginId
    }
}
impl From<ParseError> for AuthAPIError {
    fn from(error: ParseError) -> Self {
        match error {
            ParseError::InvalidEmail => AuthAPIError::InvalidEmail,
        }
    }
}
impl From<String> for AuthAPIError {
    fn from(error: String) -> Self {
        AuthAPIError::UnexpectedError(eyre!(error))
    }
}
impl From<TwoFACodeStoreError> for AuthAPIError {
    fn from(error: TwoFACodeStoreError) -> Self {
        match error {
            TwoFACodeStoreError::UnexpectedError(_) => AuthAPIError::UnexpectedError(error.into()),
            TwoFACodeStoreError::LoginAttemptIdNotFound => AuthAPIError::IncorrectCredentials,
        }
    }
}

impl From<PasswordError> for AuthAPIError {
    fn from(_error: PasswordError) -> Self {
        AuthAPIError::InvalidCredentials
    }
}
impl From<UserStoreError> for AuthAPIError {
    fn from(error: UserStoreError) -> Self {
        match error {
            UserStoreError::UserAlreadyExists => {
                AuthAPIError::UnexpectedError(eyre!("User already exists"))
            }
            UserStoreError::UserNotFound => AuthAPIError::IncorrectCredentials,
            UserStoreError::InvalidCredentials => AuthAPIError::InvalidCredentials,
            UserStoreError::UnexpectedError(e) => AuthAPIError::UnexpectedError(e),
            UserStoreError::InvalidPassword => AuthAPIError::IncorrectCredentials,
        }
    }
}
