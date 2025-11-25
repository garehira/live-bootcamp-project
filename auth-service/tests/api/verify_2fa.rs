use crate::helpers::TestApp;
use auth_service::domain::Email;
use auth_service::routes::Verify2FARequest;

use crate::helpers::get_random_email;

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app = TestApp::new().await;
    let login_body = serde_json::json!({
        "email": get_random_email(),
        // "loginAttemptId": "12345678",
        "2FACode": "123456"
    });

    let response = app.post_verify_2fa(&login_body).await;
    assert_eq!(response.status().as_u16(), 422);
    app.clean_up().await;
}

#[tokio::test]
async fn should_return_400_if_invalid_input() {
    let app = TestApp::new().await;
    let login_body = serde_json::json!({
        "email": get_random_email(),
        "loginAttemptId": "etz#", // wrong chars
        "2FACode": "123456"
    });

    let response = app.post_verify_2fa(&login_body).await;
    assert_eq!(response.status().as_u16(), 400);
    let login_body = serde_json::json!({
        "email": get_random_email(),
        "loginAttemptId": "12345678",
        "2FACode": "hu##" // must be numeric
    });

    let response = app.post_verify_2fa(&login_body).await;
    assert_eq!(response.status().as_u16(), 400);

    let response = app.post_verify_2fa(&login_body).await;
    assert_eq!(response.status().as_u16(), 400);
    let login_body = serde_json::json!({
        "email":"wrong email", // email error
        "loginAttemptId": "12345678",
        "2FACode": "123456"
    });

    let response = app.post_verify_2fa(&login_body).await;
    assert_eq!(response.status().as_u16(), 400);
    app.clean_up().await;
}
#[tokio::test]
async fn should_return_401_if_incorrect_credentials() {
    let app = TestApp::new().await;
    let r = Verify2FARequest {
        email: get_random_email(),
        login_attempt_id: "33ae76c1-6cca-437d-866c-deac249dc92e".to_string(),
        two_fa_code: "123456".to_string(),
    };
    let login_body = serde_json::to_value(&r).unwrap();

    let response = app.post_verify_2fa(&login_body).await;
    assert_eq!(response.status().as_u16(), 401);
    app.clean_up().await;
}

#[tokio::test]
async fn should_return_401_if_old_code() {
    // Call login twice. Then, attempt to call verify-fa with the 2FA code from the first login request. This should fail.
    let app = TestApp::new().await;
    let email = get_random_email();
    let user = serde_json::json!({
        "email": email,
        "password": "password123!",
        "requires2FA": true
    });

    let response = app.post_signup(&user).await;
    assert_eq!(response.status().as_u16(), 201);
    let login_response1 = app.post_login(&user).await;
    assert_eq!(login_response1.status().as_u16(), 206);
    // save the code for later...
    let user_email = Email::parse(email.clone()).unwrap();

    let saved_code = app
        .two_fa_code_store
        .read()
        .await
        .get_code(&user_email)
        .await
        .unwrap()
        .to_owned();

    let r = Verify2FARequest {
        email,
        login_attempt_id: saved_code.0.as_ref().to_string(),
        two_fa_code: saved_code.1.as_ref().to_string(),
    };

    let tweaked_2fa = serde_json::to_value(&r).unwrap();

    let verify_response1 = app.post_verify_2fa(&tweaked_2fa).await;
    assert_eq!(verify_response1.status().as_u16(), 200);

    // the second login - logout before login
    let logout_response = app.post_logout().await;
    assert_eq!(logout_response.status().as_u16(), 200);

    let login_response2 = app.post_login(&user).await;
    assert_eq!(login_response2.status().as_u16(), 206);

    // now checking on the old 2fa token, it should be stale.
    let verify_response2 = app.post_verify_2fa(&tweaked_2fa).await;
    assert_eq!(verify_response2.status().as_u16(), 401);
    app.clean_up().await;
}
