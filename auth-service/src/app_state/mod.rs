use crate::domain::data_stores::{BannedTokenStore, UserStore};
pub use crate::services::hashmap_user_store::HashmapUserStore;
use std::sync::Arc;
use tokio::sync::RwLock;

// Using a type alias to improve readability!
pub type UserStoreType = Arc<RwLock<Box<dyn UserStore>>>;
pub type BanStoreType = Arc<RwLock<Box<dyn BannedTokenStore>>>;

pub struct AppState {
    pub user_store: UserStoreType,
    pub ban_store: BanStoreType,
}

impl AppState {
    pub fn new(user_store: Box<dyn UserStore>, ban_store: BanStoreType) -> Self {
        Self {
            user_store: Arc::new(RwLock::new(user_store)),
            ban_store,
        }
    }
}
