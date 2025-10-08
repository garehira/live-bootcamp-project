use crate::helpers::TestApp;

#[tokio::test]
async fn test_verify_2fa() {
    let app = TestApp::new().await;
    let response = app.post_uri("verify-2fa").await;
    assert_eq!(response.status().as_u16(), 200);
    println!("verify_2fa ok.");
}