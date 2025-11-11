use crate::app_state::AppState;
use crate::domain::error::AuthAPIError;
use crate::util::auth;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use axum_extra::extract::CookieJar;
use serde::Deserialize;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct TokenRequest {
    pub token: String,
}

pub async fn verify_token(
    State(_state): State<Arc<AppState>>,
    _jar: CookieJar,
    Json(request): Json<TokenRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    let token = request.token;
    auth::validate_token(&token)
        .await
        .map_err(|_| AuthAPIError::InvalidToken)?;
    Ok(())
}
