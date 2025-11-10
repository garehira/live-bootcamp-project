use auth_service::services::hashmap_two_fa_code_store::HashmapTwoFACodeStore;
use auth_service::services::hashset_bannedtoken_store::HashsetBannedTokenStore;
use auth_service::util::constants::prod;
use auth_service::{app_state, Application};
use std::sync::Arc;
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    let user_store = Box::new(app_state::HashmapUserStore::default());
    let ban_store = Box::new(HashsetBannedTokenStore::default());
    let two_fa_store = Box::new(HashmapTwoFACodeStore::default());
    let app_state = app_state::AppState::new(
        user_store,
        Arc::new(RwLock::new(ban_store)),
        Arc::new(RwLock::new(two_fa_store)),
    );

    let app = Application::build(app_state, prod::APP_ADDRESS)
        .await
        .expect("Failed to build app");

    app.run().await.expect("Failed to run app");
}
