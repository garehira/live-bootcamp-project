use crate::helpers::{get_random_email, TestApp};
use auth_service::domain::email::Email;
use auth_service::routes::TwoFactorAuthResponse;
use auth_service::util::constants::JWT_COOKIE_NAME;
#[tokio::test]
async fn should_return_206_if_valid_credentials_and_2fa_enabled() {
    let app = TestApp::new().await;
    let useremail = get_random_email();

    let user = serde_json::json!({
        "email": &useremail,
        "password": "password123!",
        "requires2FA": true
    });

    let signup_response = app.post_signup(&user).await;
    assert_eq!(signup_response.status().as_u16(), 201);

    let login_response = app.post_login(&user).await;
    assert_eq!(login_response.status().as_u16(), 206);

    let json_body = login_response
        .json::<TwoFactorAuthResponse>()
        .await
        .expect("Could not deserialize response body to TwoFactorAuthResponse");

    assert_eq!(&json_body.message, &"2FA required".to_owned());

    let login_id: &str = json_body.login_attempt_id.as_ref();
    let email_of_user = Email::parse(useremail).unwrap();
    let o = app.two_fa_code_store.read().await;
    let should_id = o
        .as_ref()
        .get_code(&email_of_user)
        .await
        .unwrap()
        .0
        .as_ref();

    assert!(login_id == should_id);
}

#[tokio::test]
async fn should_return_422_if_malformed_credentials() {
    let app = TestApp::new().await;
    let test1 = serde_json::json!({
        // "email": "hermann@sachs.de",
        "password": "password123!",
    });

    let res = app.post_login(&test1).await;
    assert_eq!(res.status().as_u16(), 422);
}

#[tokio::test]
async fn should_return_400_if_invalid_input() {
    // Call the log-in route with invalid credentials and assert that a
    // 400 HTTP status code is returned along with the appropriate error message.
    let app = TestApp::new().await;

    let test_cases = [
        serde_json::json!({
            "email": "asd",
            "password": "password123!"
        }),
        serde_json::json!({
            "email": "invalidemail",
            "password": "password123!"
        }),
        serde_json::json!({
            "email": "test@example.com",
            "password": "short"
        }),
    ];

    for test_case in test_cases.iter() {
        let response = app.post_login(test_case).await;
        assert_eq!(response.status().as_u16(), 400);
    }
}
#[tokio::test]
async fn should_return_401_if_incorrect_credentials() {
    // Call the log-in route with incorrect credentials and assert
    // that a 401 HTTP status code is returned along with the appropriate error message.
    let app = TestApp::new().await;

    // First, create a new user
    let email = "herbert@example.com".to_string(); //get_random_email();
    let user = serde_json::json!({
        "email": email,
        "password": "password123!",
        "requires2FA": false
    });
    let response = app.post_signup(&user).await;
    assert_eq!(response.status().as_u16(), 201);

    // Try to login with incorrect credentials
    let invalid_credentials = serde_json::json!({
        "email": email,
        "password": "wrongpassword123!"
    });

    let response = app.post_login(&invalid_credentials).await;
    assert_eq!(response.status().as_u16(), 401);

    // Try to login with malformed credentials
    let invalid_credentials = serde_json::json!({
        "email": email,
        "password": "321!"
    });

    let response = app.post_login(&invalid_credentials).await;
    assert_eq!(response.status().as_u16(), 400);
}
#[tokio::test]
async fn validate_user_credentials() {
    let app = TestApp::new().await;

    // Create new user
    let email = get_random_email();
    let user = serde_json::json!({
        "email": email,
        "password": "password123!",
        "requires2FA": false
    });
    let response = app.post_signup(&user).await;
    assert_eq!(response.status().as_u16(), 201);

    // Try to login with correct credentials
    let valid_credentials = serde_json::json!({
        "email": email,
        "password": "password123!"
    });

    let response = app.post_login(&valid_credentials).await;
    assert_eq!(response.status().as_u16(), 200);

    // Try to login with incorrect credentials
    let valid_credentials = serde_json::json!({
        "email": email,
        "password": "password1234!"
    });

    let response = app.post_login(&valid_credentials).await;
    assert_eq!(response.status().as_u16(), 401);
}

#[tokio::test]
async fn should_return_200_if_valid_credentials_and_2fa_disabled() {
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
    response.cookies().for_each(|cookie| {
        println!("{:?}", cookie);
    });

    assert_eq!(response.status().as_u16(), 200);

    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie found");

    assert!(!auth_cookie.value().is_empty());
}
