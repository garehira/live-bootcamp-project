use crate::domain::data_stores::{UserStore, UserStoreError};
use crate::domain::email::Email;
use crate::domain::password::Password;
use crate::domain::user::User;
use std::collections::HashMap;

#[derive(Default)]
pub struct HashmapUserStore {
    users: HashMap<Email, User>,
}
#[async_trait::async_trait]
impl UserStore for HashmapUserStore {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        // Return `UserStoreError::UserAlreadyExists` if the user already exists,
        // otherwise insert the user into the hashmap and return `Ok(())`.
        if self.users.contains_key(&user.email) {
            return Err(UserStoreError::UserAlreadyExists);
        }
        self.users.insert(user.email.clone(), user);
        Ok(())
    }

    async fn get_user(&self, email: &Email) -> Result<User, UserStoreError> {
        Ok(self
            .users
            .get(email)
            .ok_or(UserStoreError::UserNotFound)?
            .clone())
    }
    async fn validate_user(
        &self,
        email: &Email,
        password: &Password,
    ) -> Result<(), UserStoreError> {
        let user = self.get_user(email).await?;
        if user.password_hash == *password {
            Ok(())
        } else {
            Err(UserStoreError::InvalidPassword)
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use secrecy::Secret;

    pub async fn test_data() -> HashmapUserStore {
        let mut store = HashmapUserStore::default();
        let user = User::new("herbert@email.com", "password123§!!", true).unwrap();
        store.add_user(user).await.unwrap();
        let user = User::new("hubert@email.com", "password123§!!", true).unwrap();
        store.add_user(user).await.unwrap();
        let user = User::new("hermann@email.com", "password123§!!", true).unwrap();
        store.add_user(user).await.unwrap();
        store
    }
    #[tokio::test]
    async fn test_add_user() {
        let mut hm = test_data().await;
        hm.add_user(User::new("herbert222@email.com", "password123+", true).unwrap())
            .await
            .expect("Failed to add user");

        let res = hm
            .add_user(User::new("herbert@email.com", "password321$", true).unwrap())
            .await;
        assert!(matches!(res, Err(UserStoreError::UserAlreadyExists)));
    }
    #[tokio::test]
    async fn test_get_user() {
        let hm = test_data();
        hm.await
            .get_user(&Email::parse("hermann@email.com".to_string()).unwrap())
            .await
            .expect("Failed to get user");
    }

    #[tokio::test]
    async fn test_validate_user() {
        let data = test_data().await;
        data.validate_user(
            &Email::unwrap("hermann@email.com"),
            &Password::parse(Secret::new("password123§!!".to_string())).unwrap(),
        )
        .await
        .expect("Failed to validate user");
        assert!(matches!(
            data.validate_user(
                &Email::unwrap("notfound@email.com"),
                &Password::parse(Secret::new("password3!".to_string())).unwrap(),
            )
            .await,
            Err(UserStoreError::UserNotFound)
        ));
        assert!(matches!(
            data.validate_user(
                &Email::unwrap("hermann@email.com"),
                &Password::parse(Secret::new("password4§".to_string())).unwrap(),
            )
            .await,
            Err(UserStoreError::InvalidPassword)
        ));
    }
}
