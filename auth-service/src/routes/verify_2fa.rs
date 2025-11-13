use crate::app_state::AppState;
use crate::domain::data_stores::{LoginAttemptId, TwoFACode};
use crate::domain::error::AuthAPIError;
use crate::domain::Email;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use axum_extra::extract::CookieJar;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize, Serialize)]
pub struct Verify2FARequest {
    pub email: String,
    #[serde(rename = "loginAttemptId")]
    pub login_attempt_id: String,
    #[serde(rename = "2FACode")]
    pub two_fa_code: String,
}

pub async fn verify_2fa(
    State(state): State<Arc<AppState>>,
    _jar: CookieJar,
    Json(request): Json<Verify2FARequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    let login_attempt_id = LoginAttemptId::parse(request.login_attempt_id)?;
    let email = Email::parse(request.email)?;
    let two_fa_code =
        TwoFACode::parse(request.two_fa_code).map_err(|_| AuthAPIError::InvalidToken)?;

    // lookup
    let t = state.two_fa_code_store.read().await;
    let found_state = t
        .as_ref()
        .get_code(&email)
        .await
        .map_err(|_| AuthAPIError::IncorrectCredentials)?;
    if *found_state != (login_attempt_id, two_fa_code) {
        return Err(AuthAPIError::IncorrectCredentials);
    }

    Ok(())
}
