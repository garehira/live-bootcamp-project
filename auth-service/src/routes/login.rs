use crate::app_state::AppState;
use crate::domain::email::Email;
use crate::domain::error::AuthAPIError;
use crate::domain::password::Password;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use axum_extra::extract::CookieJar;
use http::StatusCode;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

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
        .map_err(|_| AuthAPIError::UnexpectedError)?;

    let updated_jar = jar.add(auth_cookie);

    let user = user_store.get_user(&email).await?;

    let res = if user.requires_2fa {
        handle2fa().await?
    } else {
        handle_no_2fa().await?
    };
    Ok((updated_jar, res))
}

// New!
// async fn handle2fa(jar: CookieJar) -> Result<impl IntoResponse, AuthAPIError> {
async fn handle2fa() -> Result<(StatusCode, Json<LoginResponse>), AuthAPIError> {
    //     // TODO: Return a TwoFactorAuthResponse. The message should be "2FA required".
    //     // The login attempt ID should be "123456". We will replace this hard-coded login attempt ID soon!
    let response = Json(LoginResponse::TwoFactorAuth(TwoFactorAuthResponse {
        message: "2FA required".to_owned(),
        login_attempt_id: "123456".to_owned(),
    }));
    let statuscode = StatusCode::from_u16(206).unwrap();
    Ok((statuscode, response))
}

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
