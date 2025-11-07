use crate::app_state::AppState;
use crate::domain::error::AuthAPIError;
use crate::util::auth::validate_token;
use crate::util::constants::JWT_COOKIE_NAME;
use axum::extract::State;
use axum::response::IntoResponse;
use axum_extra::extract::CookieJar;
use std::sync::Arc;

pub async fn logout(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
) -> Result<impl IntoResponse, AuthAPIError> {
    // Retrieve JWT cookie from the `CookieJar`
    // Return AuthAPIError::MissingToken is the cookie is not found
    let cookie = jar.get(JWT_COOKIE_NAME).ok_or(AuthAPIError::MissingToken)?;

    let token = cookie.value().to_owned();

    // TODO: Validate JWT token by calling `validate_token` from the auth service.
    // If the token is valid you can ignore the returned claims for now.
    // Return AuthAPIError::InvalidToken is validation fails.
    validate_token(&token)
        .await
        .map_err(|_| AuthAPIError::InvalidToken)?;
    // Remove JWT cookie from the CookieJar
    let c = cookie.clone();
    let token = c.value();
    println!("token: {}", token);
    // ban that sucker

    let mut ban_store = state.ban_store.write().await;
    ban_store.ban(token.to_string()).await;
    let _ = jar.remove(c);

    Ok(())
}
