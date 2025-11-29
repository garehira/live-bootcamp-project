use color_eyre::eyre::{Context, Result};
use redis::{Commands, Connection};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::domain::data_stores::{BannedTokenStore, BannedTokenStoreError};
use crate::util::auth::TOKEN_TTL_SECONDS;

pub struct RedisBannedTokenStore {
    conn: Arc<RwLock<Connection>>,
}

impl RedisBannedTokenStore {
    #[tracing::instrument(name = "new redis", skip_all)]
    pub fn new(conn: Arc<RwLock<Connection>>) -> Self {
        Self { conn }
    }
}

#[async_trait::async_trait]
impl BannedTokenStore for RedisBannedTokenStore {
    #[tracing::instrument(name = "add token", skip_all)]
    async fn add_token(&mut self, token: String) -> Result<(), BannedTokenStoreError> {
        let key = get_key(&token);
        let mut red = self.conn.write().await;

        red.set_ex::<_, _, ()>(&key, true, TOKEN_TTL_SECONDS as u64)
            .wrap_err("failed to set banned token in Redis")
            .map_err(BannedTokenStoreError::UnexpectedError)?;

        Ok(())
    }
    #[tracing::instrument(name = "contains token", skip_all)]
    async fn contains_token(&self, token: &str) -> Result<bool, BannedTokenStoreError> {
        // Check if the token exists by calling the exists method on the Redis connection
        self.conn
            .write()
            .await
            .exists(&get_key(&token))
            .wrap_err("failed to check if token exists in Redis")
            .map_err(BannedTokenStoreError::UnexpectedError)
    }
}

// We are using a key prefix to prevent collisions and organize data!
const BANNED_TOKEN_KEY_PREFIX: &str = "banned_token:";
#[tracing::instrument(name = "get_key", skip_all)]
fn get_key(token: &str) -> String {
    format!("{}{}", BANNED_TOKEN_KEY_PREFIX, token)
}
