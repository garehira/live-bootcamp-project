use crate::app_state::AppState;
use crate::domain::email::Email;
use crate::domain::error::AuthAPIError;
use crate::domain::password::Password;
use crate::domain::user::User;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize)]
pub struct SignupRequest {
    pub email: String,
    pub password: String,
    #[serde(rename = "requires2FA")]
    pub requires_2fa: bool,
}

pub async fn signup(
    State(state): State<Arc<AppState>>,
    Json(request): Json<SignupRequest>,
) -> impl IntoResponse {
    // Create a new `User` instance using data in the `request`
    let ep = Email::parse(request.email);
    if ep.is_err() {
        return AuthAPIError::InvalidCredentials.into_response();
    }
    let email = ep.unwrap();

    let pr = Password::parse(request.password);
    if pr.is_err() {
        return AuthAPIError::InvalidCredentials.into_response();
    }
    let password = pr.unwrap();

    let mut user_store = state.user_store.write().await;
    if user_store.get_user(&email).await.is_ok() {
        return AuthAPIError::UserAlreadyExists.into_response();
    }

    let user = User::new2(email, password, request.requires_2fa);

    if user_store.add_user(user).await.is_err() {
        return AuthAPIError::UnexpectedError.into_response();
    }

    let response = Json(SignupResponse {
        message: "User created successfully!".to_string(),
    });

    (StatusCode::CREATED, response).into_response()
}

//...

#[derive(Serialize, Debug, Deserialize, PartialEq, Eq)]
pub struct SignupResponse {
    pub message: String,
}
