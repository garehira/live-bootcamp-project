use crate::domain::data_stores::UserStoreError;
use crate::domain::email::ParseError;
use crate::domain::password::PasswordError;
use crate::ErrorResponse;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;

pub enum AuthAPIError {
    UserAlreadyExists,
    InvalidCredentials,
    IncorrectCredentials,
    InvalidEmail,
    UnexpectedError,
    MalformedRequest,
    MissingToken,
    InvalidToken,
}
impl IntoResponse for AuthAPIError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthAPIError::UserAlreadyExists => (StatusCode::CONFLICT, "User already exists"),
            AuthAPIError::InvalidCredentials => (StatusCode::BAD_REQUEST, "Invalid credentials"),
            AuthAPIError::MalformedRequest => {
                (StatusCode::UNPROCESSABLE_ENTITY, "Malformed Request")
            }

            AuthAPIError::UnexpectedError => {
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
impl From<ParseError> for AuthAPIError {
    fn from(error: ParseError) -> Self {
        match error {
            ParseError::InvalidEmail => AuthAPIError::InvalidEmail,
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
            UserStoreError::UserAlreadyExists => AuthAPIError::UnexpectedError,
            UserStoreError::UserNotFound => AuthAPIError::UnexpectedError,
            UserStoreError::InvalidCredentials => AuthAPIError::InvalidCredentials,
            UserStoreError::UnexpectedError => AuthAPIError::UnexpectedError,
            UserStoreError::InvalidPassword => AuthAPIError::IncorrectCredentials,
        }
    }
}
