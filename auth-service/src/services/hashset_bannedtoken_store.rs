use crate::domain::data_stores::{BannedTokenStore, BannedTokenStoreError};
use std::collections::HashSet;

#[derive(Default)]
pub struct HashsetBannedTokenStore {
    banned_tokens: HashSet<String>,
}

#[async_trait::async_trait]
impl BannedTokenStore for HashsetBannedTokenStore {
    async fn add_token(&mut self, token: String) -> Result<(), BannedTokenStoreError> {
        self.banned_tokens.insert(token);
        Ok(())
    }

    async fn contains_token(&self, token: &str) -> Result<bool, BannedTokenStoreError> {
        Ok(self.banned_tokens.contains(token))
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[tokio::test]
    pub async fn test_insert_and_contain() {
        let mut store = HashsetBannedTokenStore::default();
        let t1 = "token1".to_string();
        store.add_token(t1.clone()).await.unwrap();
        assert!(store.contains_token(&t1).await.unwrap());
        assert!(!store.contains_token(&"token2".to_string()).await.unwrap());
    }
}
