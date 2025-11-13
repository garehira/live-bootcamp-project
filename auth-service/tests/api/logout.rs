use crate::helpers::{get_random_email, TestApp};
use auth_service::util::constants::JWT_COOKIE_NAME;
use reqwest::Url;

#[tokio::test]
async fn should_return_400_if_jwt_cookie_missing() {
    let app = TestApp::new().await;
    // not adding any cookie
    // no body
    let no_body = serde_json::json!({});
    let response = app.post("logout", &no_body).await;
    assert_eq!(response.status().as_u16(), 400);
}

#[tokio::test]
async fn should_return_401_if_invalid_token() {
    let app = TestApp::new().await;

    // add invalid cookie
    app.cookie_jar.add_cookie_str(
        &format!(
            "{}=invalid; HttpOnly; SameSite=Lax; Secure; Path=/",
            JWT_COOKIE_NAME
        ),
        &Url::parse("http://127.0.0.1").expect("Failed to parse URL"),
    );
    // no body
    let no_body = serde_json::json!({});
    let response = app.post("logout", &no_body).await;
    assert_eq!(response.status().as_u16(), 401);
}

#[tokio::test]
async fn should_return_200_if_valid_jwt_cookie() {
    let app = TestApp::new().await;
    let random_email = get_random_email();

    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123!",
        "requires2FA": false
    });

    let response = app.post_signup(&signup_body).await;
    assert_eq!(response.status().as_u16(), 201);

    let login_body = serde_json::json!({
        "email": random_email,
        "password": "password123!",
    });

    let response = app.post_login(&login_body).await;
    assert_eq!(response.status().as_u16(), 200);

    // verify JWT cookie exists
    let jwt_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("JWT cookie should exist after login");

    // now logout
    // let no_body = serde_json::json!({});
    // let response = app.post("logout", &no_body).await;
    let response = app.post_logout().await;
    assert_eq!(response.status().as_u16(), 200);
    // cookie shall be deleted
    assert!(response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .is_none());

    println!("token: {}", jwt_cookie.value());
    let token_banned = app
        .banned_token
        .read()
        .await
        .is_banned(&jwt_cookie.value().to_string())
        .await;
    assert!(token_banned);

    // are we really logged out?
    // let no_body = serde_json::json!({});
    // let response = app.post("logout", &no_body).await;
    // assert_eq!(response.status().as_u16(), 400); // cookie should be missing
}
