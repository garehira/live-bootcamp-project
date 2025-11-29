use color_eyre::eyre::Result;
use std::ops::Deref;
use thiserror::Error;

#[derive(sqlx::Type)]
#[sqlx(transparent)]
#[derive(sqlx::FromRow, Debug, PartialEq, Clone, Hash, Eq)]
pub struct Email(String);
#[derive(Debug, Error, Clone)]
pub enum ParseError {
    #[error("InvalidEmail")]
    InvalidEmail,
}

impl Email {
    pub fn parse(email: String) -> Result<Self, ParseError> {
        if !email.contains('@') {
            return Err(ParseError::InvalidEmail);
        }
        Ok(Email(email))
    }

    pub fn unwrap(email: &str) -> Self {
        Self::parse(email.to_string()).unwrap()
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Deref for Email {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
//*** make password type
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_email() {
        let email = "test@example.com".to_string();
        let result = Email::parse(email.clone());
        assert!(result.is_ok());
        assert_eq!(result.unwrap().as_ref(), email);
    }

    #[test]
    fn test_invalid_email() {
        let email = "invalid-email".to_string();
        let result = Email::parse(email);
        assert!(matches!(result, Err(ParseError::InvalidEmail)));
    }
}
