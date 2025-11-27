use auth_service::services::data_stares::postgres_user_store::PostgresUserStore;
use auth_service::services::data_stares::redis_banned_token_store::RedisBannedTokenStore;
use auth_service::services::hashmap_two_fa_code_store::HashmapTwoFACodeStore;
use auth_service::services::mock_email_client::MockEmailClient;
use auth_service::util::constants::{prod, DATABASE_URL, REDIS_HOST_NAME};
use auth_service::{app_state, get_postgres_pool, get_redis_client, Application};
use sqlx::PgPool;
use std::sync::Arc;
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    let pg_pool = configure_postgresql().await;
    let redis_pool = Arc::new(RwLock::new(configure_redis()));

    let user_store = Box::new(PostgresUserStore::new(pg_pool));
    let ban_store = Box::new(RedisBannedTokenStore::new(redis_pool));
    let two_fa_store = Box::new(HashmapTwoFACodeStore::default());
    let email_client = Box::new(MockEmailClient::default());
    let app_state = app_state::AppState::new(
        Arc::new(RwLock::new(user_store)),
        Arc::new(RwLock::new(ban_store)),
        Arc::new(RwLock::new(two_fa_store)),
        Arc::new(RwLock::new(email_client)),
    );

    let app = Application::build(app_state, prod::APP_ADDRESS)
        .await
        .expect("Failed to start app");

    app.run().await.expect("Failed to run app");
}
async fn configure_postgresql() -> PgPool {
    // Create a new database connection pool
    println!("Configuring database at {}", &DATABASE_URL.to_string());
    let pg_pool = get_postgres_pool(&DATABASE_URL)
        .await
        .expect("Failed to create Postgres connection pool! ");
    println!("database running.");

    // Run database migrations against our test database!
    sqlx::migrate!()
        .run(&pg_pool)
        .await
        .expect("Failed to run migrations");

    pg_pool
}

pub fn configure_redis() -> redis::Connection {
    get_redis_client(REDIS_HOST_NAME.to_owned())
        .expect("Failed to get Redis client")
        .get_connection()
        .expect("Failed to get Redis connection")
}
