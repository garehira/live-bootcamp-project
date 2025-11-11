use auth_service::app_state::{AppState, TwoFACodeStoreType};
use auth_service::app_state::{BanStoreType, EmailClientType};
use auth_service::services::hashmap_two_fa_code_store::HashmapTwoFACodeStore;
use auth_service::services::hashmap_user_store::HashmapUserStore;
use auth_service::services::hashset_bannedtoken_store::HashsetBannedTokenStore;
use auth_service::services::mock_email_client::MockEmailClient;
use auth_service::util::constants::test;
use auth_service::Application;
use reqwest::cookie::Jar;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

pub struct TestApp {
    pub address: String,
    pub cookie_jar: Arc<Jar>,
    pub http_client: reqwest::Client,
    pub banned_token: BanStoreType,
    pub two_fa_code_store: TwoFACodeStoreType,
    // pub user_store:HashmapUserStore,
}

impl TestApp {
    pub async fn new() -> Self {
        let user_store = Box::new(HashmapUserStore::default());
        let banned_token: BanStoreType =
            Arc::new(RwLock::new(Box::new(HashsetBannedTokenStore::default())));
        let two_fa_store: TwoFACodeStoreType =
            Arc::new(RwLock::new(Box::new(HashmapTwoFACodeStore::default())));
        let email_client: EmailClientType =
            Arc::new(RwLock::new(Box::new(MockEmailClient::default())));
        let app_state = AppState::new(
            user_store,
            Arc::clone(&banned_token),
            Arc::clone(&two_fa_store),
            Arc::clone(&email_client),
        );

        let app = Application::build(app_state, test::APP_ADDRESS)
            .await
            .expect("Failed to build app");

        let address = format!("http://{}", app.address.clone());

        // Run the auth service in a separate async task
        // to avoid blocking the main test thread.
        #[allow(clippy::let_underscore_future)]
        let _ = tokio::spawn(app.run());

        // Create a Reqwest http client instance
        let cookie_jar = Arc::new(Jar::default());
        let http_client = reqwest::Client::builder()
            .cookie_provider(cookie_jar.clone())
            .build()
            .unwrap();

        // Create a new ` TestApp ` instance and return it
        TestApp {
            address,
            cookie_jar,
            http_client,
            banned_token,
            two_fa_code_store: two_fa_store,
        }
    }
    pub async fn post_login<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.http_client
            .post(&format!("{}/login", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_verify_token<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.post("verify-token", &body).await
    }

    pub async fn post<Body>(&self, uri: &str, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.http_client
            .post(&format!("{}/{}", &self.address, uri))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_signup<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.http_client
            .post(&format!("{}/signup", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn get_root(&self) -> reqwest::Response {
        self.http_client
            .get(&format!("{}/", &self.address))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_uri(&self, uri: &str) -> reqwest::Response {
        self.http_client
            .post(&format!("{}/{}", &self.address, uri))
            .send()
            .await
            .expect("Failed to execute request.")
    }
}

pub fn get_random_email() -> String {
    format!("{}@example.com", Uuid::new_v4())
}
