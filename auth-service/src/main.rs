use auth_service::util::constants::prod;
use auth_service::{app_state, Application};

#[tokio::main]
async fn main() {
    let user_store = Box::new(app_state::HashmapUserStore::default());
    let app_state = app_state::AppState::new(user_store);

    let app = Application::build(app_state, prod::APP_ADDRESS)
        .await
        .expect("Failed to build app");

    app.run().await.expect("Failed to run app");
}
