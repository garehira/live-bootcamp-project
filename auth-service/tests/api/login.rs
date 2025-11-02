use crate::helpers::TestApp;

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
