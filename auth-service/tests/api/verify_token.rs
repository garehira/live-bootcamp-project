use crate::helpers::TestApp;

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app = TestApp::new().await;
    let test1 = serde_json::json!({
        // "email": "hermann@sachs.de",
        "password": "password123!",
    });

    let response = app.post_verify_token(&test1).await;
    assert_eq!(response.status().as_u16(), 422);
    app.clean_up().await;
}
#[tokio::test]
async fn should_return_200_valid_token() {
    let app = TestApp::new().await;
    let test1 = serde_json::json!({
        "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiI0NWYxMDQxYy0wM2RiLTQ2ZTEtODMzNS1hYjUyZTcxYWJmZDFAZXhhbXBsZS5jb20iLCJleHAiOjE3NjI0NDk1NzR9.d-DZlvfNIUEFD3ertZUcHHChQrGdss1lU7h1JOBVtd8; HttpOnly; SameSite=Lax; Path=/",
    });
    let response = app.post_verify_token(&test1).await;
    assert_eq!(response.status().as_u16(), 401);
    app.clean_up().await;
}

#[tokio::test]
async fn should_return_401_if_invalid_token() {
    let app = TestApp::new().await;
    let test1 = serde_json::json!({
        "token": "invalid!",
    });
    let response = app.post_verify_token(&test1).await;
    assert_eq!(response.status().as_u16(), 401);
    app.clean_up().await;
}
