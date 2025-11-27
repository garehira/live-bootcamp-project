use auth_service::app_state::{AppState, TwoFACodeStoreType, UserStoreType};
use auth_service::app_state::{BanStoreType, EmailClientType};
use auth_service::services::data_stares::postgres_user_store::PostgresUserStore;
use auth_service::services::data_stares::redis_two_fa_code_store::RedisTwoFACodeStore;
use auth_service::services::hashset_bannedtoken_store::HashsetBannedTokenStore;
use auth_service::services::mock_email_client::MockEmailClient;
use auth_service::util::constants::{test, DATABASE_URL, REDIS_HOST_NAME};
use auth_service::{get_postgres_pool, get_redis_client, Application};
use reqwest::cookie::Jar;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

pub struct TestApp {
    pub address: String,
    pub cookie_jar: Arc<Jar>,
    pub http_client: reqwest::Client,
    pub banned_token: BanStoreType,
    pub two_fa_code_store: TwoFACodeStoreType,
    pub db_name: String,
    cleanup_called: bool,
    // pub user_store:HashmapUserStore,
}

impl Drop for TestApp {
    fn drop(&mut self) {
        if !self.cleanup_called {
            panic!("TestApp instance dropped without calling clean_up()! This may leave behind test databases.");
        }
    }
}

impl TestApp {
    pub async fn clean_up(mut self) {
        delete_database(&self.db_name).await;
        self.cleanup_called = true;
    }

    pub async fn new() -> Self {
        // We are creating a new database for each test case, and we need to ensure each database has a unique name!
        let db_name = Uuid::new_v4().to_string();

        let pg_pool = configure_postgresql(&db_name).await;
        let redis = configure_redis();

        let user_store: UserStoreType =
            Arc::new(RwLock::new(Box::new(PostgresUserStore::new(pg_pool))));

        let banned_token: BanStoreType =
            Arc::new(RwLock::new(Box::new(HashsetBannedTokenStore::default())));
        let two_fa_store: TwoFACodeStoreType = Arc::new(RwLock::new(Box::new(
            RedisTwoFACodeStore::new(Arc::new(RwLock::new(redis))),
        )));
        let email_client: EmailClientType =
            Arc::new(RwLock::new(Box::new(MockEmailClient::default())));
        let app_state = AppState::new(
            Arc::clone(&user_store),
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
            cleanup_called: false,
            db_name,
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

    pub async fn post_logout(&self) -> reqwest::Response {
        self.post("logout", &"".to_string()).await
    }

    pub async fn post_verify_2fa<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.post("verify-2fa", &body).await
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
}
pub fn configure_redis() -> redis::Connection {
    get_redis_client(REDIS_HOST_NAME.to_owned())
        .expect("Failed to get Redis client")
        .get_connection()
        .expect("Failed to get Redis connection")
}

pub fn get_random_email() -> String {
    format!("{}@example.com", Uuid::new_v4())
}

async fn configure_postgresql(db_name: &String) -> PgPool {
    // configure_database(&postgresql_conn_url, &db_name).await;
    configure_database(&DATABASE_URL, &db_name).await;

    let postgresql_conn_url_with_db = format!("{}/{}", &DATABASE_URL.to_string(), &db_name);

    // Create a new connection pool and return it
    get_postgres_pool(&postgresql_conn_url_with_db)
        .await
        .expect("Failed to create Postgres connection pool!")
}
async fn configure_database(db_conn_string: &str, db_name: &str) {
    // Create database connection
    let connection = PgPoolOptions::new()
        .connect(db_conn_string)
        .await
        .expect("Failed to create Postgres connection pool.");

    // Create a new database
    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, db_name).as_str())
        .await
        .expect("Failed to create database.");

    // Connect to new database
    let db_conn_string = format!("{}/{}", db_conn_string, db_name);

    let connection = PgPoolOptions::new()
        .connect(&db_conn_string)
        .await
        .expect("Failed to create Postgres connection pool.");

    // Run migrations against new database
    sqlx::migrate!()
        .run(&connection)
        .await
        .expect("Failed to migrate the database");
}
async fn delete_database(db_name: &str) {
    let postgresql_conn_url: String = DATABASE_URL.to_owned();

    let connection_options = PgConnectOptions::from_str(&postgresql_conn_url)
        .expect("Failed to parse PostgreSQL connection string");

    let mut connection = PgConnection::connect_with(&connection_options)
        .await
        .expect("Failed to connect to Postgres");

    // Kill any active connections to the database
    connection
        .execute(
            format!(
                r#"
                SELECT pg_terminate_backend(pg_stat_activity.pid)
                FROM pg_stat_activity
                WHERE pg_stat_activity.datname = '{}'
                  AND pid <> pg_backend_pid();
        "#,
                db_name
            )
            .as_str(),
        )
        .await
        .expect("Failed to drop the database.");

    // Drop the database
    connection
        .execute(format!(r#"DROP DATABASE "{}";"#, db_name).as_str())
        .await
        .expect("Failed to drop the database.");
}
