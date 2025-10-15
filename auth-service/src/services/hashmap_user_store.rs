use std::collections::HashMap;

use crate::domain::user::User;

#[derive(Debug, PartialEq)]
pub enum UserStoreError {
    UserAlreadyExists,
    UserNotFound,
    InvalidCredentials,
    UnexpectedError,
}

#[derive(Default)]
pub struct HashmapUserStore {
    users: HashMap<String, User>,
}
impl HashmapUserStore {
    pub fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        // Return `UserStoreError::UserAlreadyExists` if the user already exists,
        // otherwise insert the user into the hashmap and return `Ok(())`.
        if self.users.contains_key(&user.email) {
            return Err(UserStoreError::UserAlreadyExists);
        }
        self.users.insert(user.email.clone(), user);
        Ok(())
    }

    pub fn get_user(&self, email: String) -> Result<&User, UserStoreError> {
        self.users.get(&email).ok_or(UserStoreError::UserNotFound)
    }
    pub fn validate_user(&self, email: &str, password: &str) -> Result<(), UserStoreError> {
        let user = self.get_user(email.to_string())?;
        if user.password == password {
            Ok(())
        } else {
            Err(UserStoreError::InvalidCredentials)
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    pub fn test_data() -> HashmapUserStore {
        let mut store = HashmapUserStore::default();
        let user = User::new(
            "herbert@email.com".to_string(),
            "password".to_string(),
            true,
        );
        store.add_user(user).unwrap();
        let user = User::new(
            "hubert@email.com".to_string(),
            "password2".to_string(),
            true,
        );
        store.add_user(user).unwrap();
        let user = User::new(
            "hermann@email.com".to_string(),
            "password3".to_string(),
            true,
        );
        store.add_user(user).unwrap();
        store
    }
    #[tokio::test]
    async fn test_add_user() {
        let mut hm = test_data();
        hm.add_user(User::new(
            "herbert222@email.com".to_string(),
            "password".to_string(),
            true,
        ))
        .expect("Failed to add user");

        assert!(matches!(
            hm.add_user(User::new(
                "herbert@email.com".to_string(),
                "password".to_string(),
                true,
            )),
            Err(UserStoreError::UserAlreadyExists)
        ));
    }
    #[tokio::test]
    async fn test_get_user() {
        let hm = test_data();
        hm.get_user("hermann@email.com".to_string())
            .expect("Failed to get user");
    }

    #[tokio::test]
    async fn test_validate_user() {
        let data = test_data();
        data.validate_user("hermann@email.com", "password3")
            .expect("Failed to validate user");
        assert!(matches!(
            data.validate_user("notfound@email.com", "password3"),
            Err(UserStoreError::UserNotFound)
        ));
        assert!(matches!(
            data.validate_user("hermann@email.com", "password4"),
            Err(UserStoreError::InvalidCredentials)
        ));
    }
}
