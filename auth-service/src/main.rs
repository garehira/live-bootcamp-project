use auth_service::domain::data_stores::UserStore;
use auth_service::domain::user::User;
use auth_service::services::hashmap_two_fa_code_store::HashmapTwoFACodeStore;
use auth_service::services::hashset_bannedtoken_store::HashsetBannedTokenStore;
use auth_service::services::mock_email_client::MockEmailClient;
use auth_service::util::constants::{prod, DATABASE_URL};
use auth_service::{app_state, get_postgres_pool, Application};
use sqlx::PgPool;
use std::sync::Arc;
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    let pg_pool = configure_postgresql().await;

    let mut user_store = Box::new(app_state::HashmapUserStore::default());
    user_store
        .add_user(User::new("frank@sinst.com", "password123!", true).unwrap())
        .await
        .expect("TODO: panic message");
    let ban_store = Box::new(HashsetBannedTokenStore::default());
    let two_fa_store = Box::new(HashmapTwoFACodeStore::default());
    let email_client = Box::new(MockEmailClient::default());
    let app_state = app_state::AppState::new(
        user_store,
        Arc::new(RwLock::new(ban_store)),
        Arc::new(RwLock::new(two_fa_store)),
        Arc::new(RwLock::new(email_client)),
    );

    let app = Application::build(app_state, prod::APP_ADDRESS)
        .await
        .expect("Failed to build app");

    app.run().await.expect("Failed to run app");
}
async fn configure_postgresql() -> PgPool {
    // Create a new database connection pool
    println!("Configuring database at {}", &DATABASE_URL.to_string());
    let pg_pool = get_postgres_pool(&DATABASE_URL)
        .await
        .expect("Failed to create Postgres connection pool!");
    println!("database running.");

    // Run database migrations against our test database!
    sqlx::migrate!()
        .run(&pg_pool)
        .await
        .expect("Failed to run migrations");

    pg_pool
}
