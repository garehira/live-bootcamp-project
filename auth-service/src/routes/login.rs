use crate::app_state::AppState;
use crate::domain::email::Email;
use crate::domain::error::AuthAPIError;
use crate::domain::password::Password;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::Deserialize;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(request): Json<LoginRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    let email = Email::parse(request.email)?;
    let password = Password::parse(request.password)?;

    let user_store = &state.user_store.read().await;
    user_store.validate_user(&email, &password).await?;

    Ok(StatusCode::OK.into_response())
}
