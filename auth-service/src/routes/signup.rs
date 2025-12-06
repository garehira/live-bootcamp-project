use crate::app_state::AppState;
use crate::domain::email::Email;
use crate::domain::error::AuthAPIError;
use crate::domain::password::Password;
use crate::domain::user::User;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use secrecy::{ExposeSecret, Secret};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize)]
pub struct SignupRequest {
    pub email: Secret<String>,
    pub password: Secret<String>,
    #[serde(rename = "requires2FA")]
    pub requires_2fa: bool,
}
#[tracing::instrument(name = "Signup", skip_all)]
pub async fn signup(
    State(state): State<Arc<AppState>>,
    Json(request): Json<SignupRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    // Create a new `User` instance using data in the `request`
    let email = Email::parse(request.email.expose_secret().clone())
        .map_err(|_| AuthAPIError::InvalidCredentials)?;
    let password =
        Password::parse(request.password).map_err(|_| AuthAPIError::InvalidCredentials)?;

    let mut user_store = state.user_store.write().await;
    if user_store.get_user(&email).await.is_ok() {
        return Err(AuthAPIError::UserAlreadyExists);
    }

    let user = User::new2(email, password, request.requires_2fa);

    user_store
        .add_user(user)
        .await
        .map_err(|e| AuthAPIError::UnexpectedError(e.into()))?;

    let response = Json(SignupResponse {
        message: "User created successfully!".to_string(),
    });

    Ok((StatusCode::CREATED, response))
}

//...

#[derive(Serialize, Debug, Deserialize, PartialEq, Eq)]
pub struct SignupResponse {
    pub message: String,
}
