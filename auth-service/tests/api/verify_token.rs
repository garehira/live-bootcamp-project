use crate::helpers::TestApp;

#[tokio::test]
async fn test_verify_token() {
    let app = TestApp::new().await;
    let response = app.post_uri("verify-token").await;
    assert_eq!(response.status().as_u16(), 200);
    println!("verify_token ok.");
}
