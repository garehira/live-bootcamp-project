use std::sync::Arc;
use tokio::sync::RwLock;

pub use crate::services::hashmap_user_store::HashmapUserStore;

// Using a type alias to improve readability!
pub type UserStoreType = Arc<RwLock<HashmapUserStore>>;

pub struct AppState {
    pub user_store: UserStoreType,
}

impl AppState {
    pub fn new(user_store: HashmapUserStore) -> Self {
        Self {
            user_store: Arc::new(RwLock::new(user_store)),
        }
    }
}
