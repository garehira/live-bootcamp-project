use crate::helpers::TestApp;

#[tokio::test]
async fn test_signup() {
    let app = TestApp::new().await;
    let response = app.post_uri("signup").await;
    assert_eq!(response.status().as_u16(), 200);
    println!("signup ok.");
}
