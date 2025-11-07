use crate::domain::email::Email;
use crate::domain::password::Password;
use crate::domain::user::User;

#[derive(Debug, PartialEq)]
pub enum UserStoreError {
    UserAlreadyExists,
    UserNotFound,
    InvalidCredentials,
    InvalidPassword,
    UnexpectedError,
}
#[async_trait::async_trait]
pub trait UserStore: Send + Sync {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError>;
    async fn get_user<'a>(&'a self, email: &Email) -> Result<&'a User, UserStoreError>;
    async fn validate_user(&self, email: &Email, password: &Password)
        -> Result<(), UserStoreError>;
}
#[async_trait::async_trait]
pub trait BannedTokenStore: Send + Sync {
    async fn ban(&mut self, token: String) -> ();
    async fn is_banned(&self, token: &String) -> bool;
}
