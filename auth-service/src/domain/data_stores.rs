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
    async fn get_user(&self, email: &Email) -> Result<User, UserStoreError>;
    async fn validate_user(&self, email: &Email, password: &Password)
        -> Result<(), UserStoreError>;
}
#[derive(Debug, PartialEq)]
pub enum BannedTokenStoreError {
    UnexpectedError,
}
#[async_trait::async_trait]
pub trait BannedTokenStore: Send + Sync {
    async fn add_token(&mut self, token: String) -> Result<(), BannedTokenStoreError>;
    async fn contains_token(&self, token: &str) -> Result<bool, BannedTokenStoreError>;
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
    pub fn parse(id: String) -> Result<Self, uuid::Error> {
        // Use the `parse_str` function from the `uuid` crate to ensure `id` is a valid UUID
        Ok(Self(Uuid::parse_str(id.as_str())?.to_string()))
    }
    pub fn as_ref(&self) -> &str {
        &self.0
    }
}

#[test]
pub fn test_parse() {
    let inpo = uuid::Uuid::new_v4().to_string();
    println!("inpo {}", inpo);
    let l = LoginAttemptId::parse(inpo.to_owned()).unwrap();
    // let l = LoginAttemptId::parse("123456".to_string()).unwrap();
    println!("l {}", l.as_ref());
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
