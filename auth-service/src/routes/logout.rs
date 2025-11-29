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
    validate_token(&token)
        .await
        .map_err(|_| AuthAPIError::InvalidToken)?;
    // Remove JWT cookie from the CookieJar
    let c = cookie.clone();
    let token = c.value();
    println!("token: {}", token);

    // should but dont have email
    // state.two_fa_code_store.write().await.remove_code()
    // ban that sucker

    let mut ban_store = state.ban_store.write().await;
    ban_store
        .add_token(token.to_string())
        .await
        .map_err(|e| AuthAPIError::UnexpectedError(e.into()))?;
    let _ = jar.remove(c);

    Ok(())
}
