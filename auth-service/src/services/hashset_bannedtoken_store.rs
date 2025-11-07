use crate::domain::data_stores::BannedTokenStore;
use std::collections::HashSet;

#[derive(Default)]
pub struct HashsetBannedTokenStore {
    banned_tokens: HashSet<String>,
}

#[async_trait::async_trait]
impl BannedTokenStore for HashsetBannedTokenStore {
    async fn ban(&mut self, token: String) -> () {
        self.banned_tokens.insert(token);
        ()
    }

    async fn is_banned(&self, token: &String) -> bool {
        self.banned_tokens.contains(token)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[tokio::test]
    pub async fn test_insert_and_contain() {
        let mut store = HashsetBannedTokenStore::default();
        let t1 = "token1".to_string();
        store.ban(t1.clone()).await;
        assert!(store.is_banned(&t1).await);
        assert!(!store.is_banned(&"token2".to_string()).await);
    }
}
