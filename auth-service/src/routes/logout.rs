use crate::domain::error::AuthAPIError;
use crate::util::auth::validate_token;
use crate::util::constants::JWT_COOKIE_NAME;
use axum::response::IntoResponse;
use axum_extra::extract::CookieJar;

pub async fn logout(jar: CookieJar) -> Result<impl IntoResponse, AuthAPIError> {
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
    let _ = jar.remove(c);

    Ok(())
}
