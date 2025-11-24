use crate::domain::data_stores::{BannedTokenStore, TwoFACodeStore, UserStore};
use crate::domain::EmailClient;
pub use crate::services::hashmap_user_store::HashmapUserStore;
use std::sync::Arc;
use tokio::sync::RwLock;

// Using a type alias to improve readability!
pub type UserStoreType = Arc<RwLock<Box<dyn UserStore>>>;
pub type BanStoreType = Arc<RwLock<Box<dyn BannedTokenStore>>>;
pub type TwoFACodeStoreType = Arc<RwLock<Box<dyn TwoFACodeStore>>>;
pub type EmailClientType = Arc<RwLock<Box<dyn EmailClient>>>;
pub struct AppState {
    pub user_store: UserStoreType,
    pub ban_store: BanStoreType,
    pub two_fa_code_store: TwoFACodeStoreType,
    pub email_client: EmailClientType,
}

impl AppState {
    pub fn new(
        user_store: UserStoreType,
        ban_store: BanStoreType,
        two_fa_code_store: TwoFACodeStoreType,
        email_client: EmailClientType,
    ) -> Self {
        Self {
            user_store,
            ban_store,
            two_fa_code_store,
            email_client,
        }
    }
}
