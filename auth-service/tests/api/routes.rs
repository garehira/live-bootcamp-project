use crate::helpers::TestApp;

// Tokio's test macro is used to run the test in an async environment
#[tokio::test]
async fn root_returns_auth_ui() {
    let app = TestApp::new().await;

    let response = app.get_root().await;

    assert_eq!(response.status().as_u16(), 200);
    assert_eq!(response.headers().get("content-type").unwrap(), "text/html");
    println!("all ok.");
}
#[tokio::test]
async fn test_signup() {
    let app = TestApp::new().await;
    let response = app.post_uri("signup").await;
    assert_eq!(response.status().as_u16(), 200);
    println!("signup ok.");
}
#[tokio::test]
async fn test_login() {
    let app = TestApp::new().await;
    let response = app.post_uri("login").await;
    assert_eq!(response.status().as_u16(), 200);
    println!("login ok.");
}
#[tokio::test]
async fn test_verify_2fa() {
    let app = TestApp::new().await;
    let response = app.post_uri("verify-2fa").await;
    assert_eq!(response.status().as_u16(), 200);
    println!("verify_2fa ok.");
}
#[tokio::test]
async fn test_logout() {
    let app = TestApp::new().await;
    let response = app.post_uri("logout").await;
    assert_eq!(response.status().as_u16(), 200);
    println!("logout ok.");
}
#[tokio::test]
async fn test_verify_token() {
    let app = TestApp::new().await;
    let response = app.post_uri("verify-token").await;
    assert_eq!(response.status().as_u16(), 200);
    println!("verify_token ok.");
}
