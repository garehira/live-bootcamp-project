use crate::app_state::AppState;
use crate::domain::error::AuthAPIError;
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
    let email = request.email;
    if !email.contains('@') || email.is_empty() {
        return AuthAPIError::InvalidCredentials.into_response();
    }
    let password = request.password;
    if password.len() < 8 {
        return AuthAPIError::InvalidCredentials.into_response();
    }
    let user = User::new(email.clone(), password, request.requires_2fa);

    let mut user_store = state.user_store.write().await;

    if user_store.get_user(email).await.is_ok() {
        return AuthAPIError::UserAlreadyExists.into_response();
    }

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
