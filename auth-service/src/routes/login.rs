use crate::app_state::AppState;
use crate::domain::email::Email;
use crate::domain::error::AuthAPIError;
use crate::domain::password::Password;
use axum::extract::State;
use axum::response::{Html, IntoResponse};
use axum::Json;
use serde::Deserialize;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

pub async fn login(
    State(_state): State<Arc<AppState>>,
    Json(request): Json<LoginRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    let _email = Email::parse(request.email)?;
    let _password = Password::parse(request.password)?;
    Ok(Html("Login").into_response())
}
