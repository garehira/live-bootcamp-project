use std::collections::HashMap;

use crate::domain::{
    data_stores::{LoginAttemptId, TwoFACode, TwoFACodeStore, TwoFACodeStoreError},
    email::Email,
};

#[derive(Default)]
pub struct HashmapTwoFACodeStore {
    codes: HashMap<Email, (LoginAttemptId, TwoFACode)>,
}
// This trait represents the interface all concrete 2FA code stores should implement
#[async_trait::async_trait]
impl TwoFACodeStore for HashmapTwoFACodeStore {
    async fn add_code(
        &mut self,
        email: Email,
        login_attempt_id: LoginAttemptId,
        code: TwoFACode,
    ) -> Result<(), TwoFACodeStoreError> {
        // if self.codes.contains_key(&email) {
        //     return Err(TwoFACodeStoreError::UnexpectedError);
        // }
        self.codes.insert(email, (login_attempt_id, code));
        Ok(())
    }

    async fn remove_code(&mut self, email: &Email) -> Result<(), TwoFACodeStoreError> {
        if !self.codes.contains_key(email) {
            return Err(TwoFACodeStoreError::UnexpectedError);
        }
        self.codes.remove(email);
        Ok(())
    }

    async fn get_code(
        &self,
        email: &Email,
    ) -> Result<&(LoginAttemptId, TwoFACode), TwoFACodeStoreError> {
        match self.codes.get(email) {
            Some(code) => Ok(code),
            None => Err(TwoFACodeStoreError::UnexpectedError),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> (HashmapTwoFACodeStore, Email, LoginAttemptId, TwoFACode) {
        let store = HashmapTwoFACodeStore::default();
        let email = Email::parse("test@example.com".to_string()).unwrap();
        let login_attempt_id = LoginAttemptId::default();
        let code = TwoFACode::default();
        (store, email, login_attempt_id, code)
    }

    #[tokio::test]
    async fn test_add_code_success() {
        let (mut store, email, login_attempt_id, code) = setup();
        let result = store.add_code(email, login_attempt_id, code).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_remove_code_success() {
        let (mut store, email, login_attempt_id, code) = setup();
        store
            .add_code(email.clone(), login_attempt_id, code)
            .await
            .unwrap();
        let result = store.remove_code(&email).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_code_success() {
        let (mut store, email, login_attempt_id, code) = setup();
        store
            .add_code(email.clone(), login_attempt_id.clone(), code.clone())
            .await
            .unwrap();
        let result = store.get_code(&email).await;
        assert!(result.is_ok());
        let (stored_id, stored_code) = result.unwrap();
        assert_eq!(stored_id, &login_attempt_id);
        assert_eq!(stored_code, &code);
    }

    // #[tokio::test]
    // async fn test_add_code_duplicate_error() {
    //     let (mut store, email, login_attempt_id, code) = setup();
    //     store
    //         .add_code(email.clone(), login_attempt_id.clone(), code.clone())
    //         .await
    //         .unwrap();
    //     let result = store.add_code(email, login_attempt_id, code).await;
    //     assert!(matches!(result, Err(TwoFACodeStoreError::UnexpectedError)));
    // }

    #[tokio::test]
    async fn test_remove_code_not_found_error() {
        let (mut store, email, _, _) = setup();
        let result = store.remove_code(&email).await;
        assert!(matches!(result, Err(TwoFACodeStoreError::UnexpectedError)));
    }

    #[tokio::test]
    async fn test_get_code_not_found_error() {
        let (store, email, _, _) = setup();
        let result = store.get_code(&email).await;
        assert!(matches!(result, Err(TwoFACodeStoreError::UnexpectedError)));
    }
}
