use crate::domain::data_stores::{BannedTokenStore, TwoFACodeStore, UserStore};
pub use crate::services::hashmap_user_store::HashmapUserStore;
use std::sync::Arc;
use tokio::sync::RwLock;

// Using a type alias to improve readability!
pub type UserStoreType = Arc<RwLock<Box<dyn UserStore>>>;
pub type BanStoreType = Arc<RwLock<Box<dyn BannedTokenStore>>>;
pub type TwoFACodeStoreType = Arc<RwLock<Box<dyn TwoFACodeStore>>>;
pub struct AppState {
    pub user_store: UserStoreType,
    pub ban_store: BanStoreType,
    pub two_fa_code_store: TwoFACodeStoreType,
}

impl AppState {
    pub fn new(
        user_store: Box<dyn UserStore>,
        ban_store: BanStoreType,
        two_fa_code_store: TwoFACodeStoreType,
    ) -> Self {
        Self {
            user_store: Arc::new(RwLock::new(user_store)),
            ban_store,
            two_fa_code_store,
        }
    }
}
