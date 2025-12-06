use color_eyre::eyre::Context;
use redis::{Commands, Connection};
use secrecy::ExposeSecret;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::domain::{
    data_stores::{LoginAttemptId, TwoFACode, TwoFACodeStore, TwoFACodeStoreError},
    Email,
};

pub struct RedisTwoFACodeStore {
    conn: Arc<RwLock<Connection>>,
}

impl RedisTwoFACodeStore {
    #[tracing::instrument(name = "new 2fa redis", skip_all)]
    pub fn new(conn: Arc<RwLock<Connection>>) -> Self {
        Self { conn }
    }
}

#[async_trait::async_trait]
impl TwoFACodeStore for RedisTwoFACodeStore {
    #[tracing::instrument(name = "add_code", skip_all)]
    async fn add_code(
        &mut self,
        email: Email,
        login_attempt_id: LoginAttemptId,
        code: TwoFACode,
    ) -> Result<(), TwoFACodeStoreError> {
        // 1. Create a new key using the get_key helper function.
        let key = get_key(&email);
        // 2. Create a TwoFATuple instance.
        let fatuple = TwoFATuple(
            login_attempt_id.as_ref().to_owned(),
            code.as_ref().to_owned(),
        );
        // 3. Use serde_json::to_string to serialize the TwoFATuple instance into a JSON string.
        let jfa = serde_json::to_string(&fatuple)
            .wrap_err("failed to serialize 2FA tuple")
            .map_err(TwoFACodeStoreError::UnexpectedError)?;
        // Return TwoFACodeStoreError::UnexpectedError if serialization fails.
        // 4. Call the set_ex command on the Redis connection to set a new key/value pair with an expiration time (TTL).
        // The value should be the serialized 2FA tuple.
        // The expiration time should be set to TEN_MINUTES_IN_SECONDS.
        // Return TwoFACodeStoreError::UnexpectedError if casting fails or the call to set_ex fails.
        self.conn
            .write()
            .await
            .set_ex::<_, _, ()>(&key, jfa, TEN_MINUTES_IN_SECONDS)
            .wrap_err("failed to set 2FA code")
            .map_err(TwoFACodeStoreError::UnexpectedError)?;

        Ok(())
    }
    #[tracing::instrument(name = "remove code", skip_all)]
    async fn remove_code(&mut self, email: &Email) -> Result<(), TwoFACodeStoreError> {
        // 1. Create a new key using the get_key helper function.
        let key = get_key(&email);
        // 2. Call the del command on the Redis connection to delete the 2FA code entry.
        // Return TwoFACodeStoreError::UnexpectedError if the operation fails.
        self.conn
            .write()
            .await
            .del(&key)
            .wrap_err("failed to delete 2FA code")
            .map_err(TwoFACodeStoreError::UnexpectedError)
    }
    #[tracing::instrument(name = "get code", skip_all)]
    async fn get_code(
        &self,
        email: &Email,
    ) -> Result<(LoginAttemptId, TwoFACode), TwoFACodeStoreError> {
        // 1. Create a new key using the get_key helper function.
        let key = get_key(&email);
        // 2. Call the get command on the Redis connection to get the value stored for the key.
        // Return TwoFACodeStoreError::LoginAttemptIdNotFound if the operation fails.
        let code: String = self
            .conn
            .write()
            .await
            .get(key)
            .wrap_err("failed to get 2FA code")
            .map_err(|_| TwoFACodeStoreError::LoginAttemptIdNotFound)?;
        // If the operation succeeds, call serde_json::from_str to parse the JSON string into a TwoFATuple.
        let TwoFATuple(login_att, code2fa) = serde_json::from_str(&*code)
            .wrap_err("failed to get 2FA code.")
            .map_err(TwoFACodeStoreError::UnexpectedError)?;
        // Then, parse the login attempt ID string and 2FA code string into a LoginAttemptId and TwoFACode type respectively.
        let login_as_type =
            LoginAttemptId::parse(login_att).map_err(TwoFACodeStoreError::UnexpectedError)?;
        let code_as_type =
            TwoFACode::parse(code2fa).map_err(TwoFACodeStoreError::UnexpectedError)?;
        // Return TwoFACodeStoreError::UnexpectedError if parsing fails.
        Ok((login_as_type, code_as_type))
    }
}

#[derive(Serialize, Deserialize)]
struct TwoFATuple(pub String, pub String);

const TEN_MINUTES_IN_SECONDS: u64 = 600;
const TWO_FA_CODE_PREFIX: &str = "two_fa_code:";
#[tracing::instrument(name = "get key", skip_all)]
fn get_key(email: &Email) -> String {
    format!("{}{}", TWO_FA_CODE_PREFIX, email.as_ref().expose_secret())
}
