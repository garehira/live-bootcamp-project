use crate::domain::email::Email;
use crate::domain::password::Password;
use crate::domain::user::User;
use rand::Rng;
use uuid::Uuid;

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
#[async_trait::async_trait]
pub trait TwoFACodeStore: Send + Sync {
    async fn add_code(
        &mut self,
        email: Email,
        login_attempt_id: LoginAttemptId,
        code: TwoFACode,
    ) -> Result<(), TwoFACodeStoreError>;
    async fn remove_code(&mut self, email: &Email) -> Result<(), TwoFACodeStoreError>;
    async fn get_code(
        &self,
        email: &Email,
    ) -> Result<&(LoginAttemptId, TwoFACode), TwoFACodeStoreError>;
}
#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub struct LoginAttemptId(String);
impl LoginAttemptId {
    pub fn parse(id: String) -> Result<Self, String> {
        // Use the `parse_str` function from the `uuid` crate to ensure `id` is a valid UUID
        match Uuid::parse_str(id.as_str()) {
            Ok(uuid) => Ok(Self(uuid.to_string())),
            Err(_) => Err("Invalid UUID".to_string()),
        }
    }
    pub fn as_ref(&self) -> &str {
        &self.0
    }
}
impl Default for LoginAttemptId {
    fn default() -> Self {
        // Use the `uuid` crate to generate a random version 4 UUID
        Self(Uuid::new_v4().to_string())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TwoFACode(String);

impl TwoFACode {
    pub fn parse(code: String) -> Result<Self, String> {
        // Ensure `code` is a valid 6-digit code
        if code.len() != 6 {
            return Err("Invalid code".to_string());
        }
        Ok(Self(code))
    }
    pub fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Default for TwoFACode {
    fn default() -> Self {
        // Use the `rand` crate to generate a random 2FA code.
        // The code should be 6 digits (ex: 834629)
        let code = rand::thread_rng().gen_range(100000..=999999).to_string();
        Self(code)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum TwoFACodeStoreError {
    LoginAttemptIdNotFound,
    UnexpectedError,
}
