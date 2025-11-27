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
    {
        // restrict life span of lock
        let lock_code_store = state.two_fa_code_store.read().await;
        let found_state = lock_code_store
            .get_code(&email)
            .await
            .map_err(|_| AuthAPIError::IncorrectCredentials)?;
        // .to_owned(); // this is the easy way to end the life of lock and get the data out.

        if found_state != (login_attempt_id, two_fa_code) {
            return Err(AuthAPIError::IncorrectCredentials);
        }
    } // end of life for lock
      // remove state
    state
        .two_fa_code_store
        .write()
        .await
        .remove_code(&email)
        .await?;

    Ok(())
}
