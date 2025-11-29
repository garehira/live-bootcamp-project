use crate::app_state::AppState;
use crate::domain::data_stores::{LoginAttemptId, TwoFACode};
use crate::domain::email::Email;
use crate::domain::error::AuthAPIError;
use crate::domain::password::Password;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use axum_extra::extract::CookieJar;
use color_eyre::eyre::eyre;
use http::StatusCode;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}
#[tracing::instrument(name = "Login", skip_all)]
pub async fn login(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
    Json(request): Json<LoginRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    let email = Email::parse(request.email)?;
    let password = Password::parse(request.password)?;

    let user_store = &state.user_store.read().await;
    user_store.validate_user(&email, &password).await?;

    let auth_cookie = crate::util::auth::generate_auth_cookie(&email)
        .map_err(|e| AuthAPIError::UnexpectedError(eyre!(e)))?;

    let updated_jar = jar.add(auth_cookie);
    let user = user_store.get_user(&email).await?;
    let res = if user.requires_2fa {
        handle_2fa(&user.email, &state).await?
    } else {
        handle_no_2fa().await?
    };
    Ok((updated_jar, res))
}

#[tracing::instrument(name = "handle_2fa", skip_all)]
async fn handle_2fa(
    email: &Email,
    app_state: &Arc<AppState>,
) -> Result<(StatusCode, Json<LoginResponse>), AuthAPIError> {
    // First, we must generate a new random login attempt ID and 2FA code
    let login_attempt_id = LoginAttemptId::default();
    let two_fa_code = TwoFACode::default();

    app_state
        .email_client
        .write()
        .await
        .send_email(email, "Here is your 2FA Token", two_fa_code.as_ref())
        .await
        .map_err(|e| AuthAPIError::UnexpectedError(e))?;

    let mut write_lock = app_state.two_fa_code_store.write().await;

    write_lock
        .as_mut()
        .add_code(email.clone(), login_attempt_id.clone(), two_fa_code)
        .await?;

    std::mem::drop(write_lock);
    // Finally, we need to return the login attempt ID to the client
    let response = Json(LoginResponse::TwoFactorAuth(TwoFactorAuthResponse {
        message: "2FA required".to_string(),
        login_attempt_id: login_attempt_id.as_ref().to_string(), // Add the generated login attempt ID
    }));
    let status_code = StatusCode::from_u16(206).unwrap();
    Ok((status_code, response))
}

#[tracing::instrument(name = "handle_no_2fa", skip_all)]
async fn handle_no_2fa() -> Result<(StatusCode, Json<LoginResponse>), AuthAPIError> {
    let response = Json(LoginResponse::RegularAuth);
    let statuscode = StatusCode::OK;
    Ok((statuscode, response))
}

// The login route can return 2 possible success responses.
// This enum models each response!
#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum LoginResponse {
    RegularAuth,
    TwoFactorAuth(TwoFactorAuthResponse),
}

// If a user requires 2FA, this JSON body should be returned!
#[derive(Debug, Serialize, Deserialize)]
pub struct TwoFactorAuthResponse {
    pub message: String,
    #[serde(rename = "loginAttemptId")]
    pub login_attempt_id: String,
}
