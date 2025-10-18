use crate::domain::data_stores::UserStore;
pub use crate::services::hashmap_user_store::HashmapUserStore;
use std::sync::Arc;
use tokio::sync::RwLock;

// Using a type alias to improve readability!
pub type UserStoreType = Arc<RwLock<Box<dyn UserStore>>>;

pub struct AppState {
    pub user_store: UserStoreType,
}

impl AppState {
    pub fn new(user_store: Box<dyn UserStore>) -> Self {
        Self {
            user_store: Arc::new(RwLock::new(user_store)),
        }
    }
}
