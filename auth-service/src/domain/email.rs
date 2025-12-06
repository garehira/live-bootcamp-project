use color_eyre::eyre::Result;
use secrecy::{ExposeSecret, Secret};
use std::hash::Hash;
use std::ops::Deref;
use thiserror::Error;

#[derive(Debug, Clone)] // Updated!
pub struct Email(Secret<String>); // Updated!

// New!
impl PartialEq for Email {
    fn eq(&self, other: &Self) -> bool {
        self.0.expose_secret() == other.0.expose_secret()
    }
}

// New!
impl Hash for Email {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.expose_secret().hash(state);
    }
}

// New!
impl Eq for Email {}

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
        Ok(Email(Secret::new(email)))
    }

    pub fn unwrap(email: &str) -> Self {
        Self::parse(email.to_string()).unwrap()
    }
}

impl AsRef<Secret<String>> for Email {
    fn as_ref(&self) -> &Secret<String> {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_email() {
        let email = "test@example.com".to_string();
        let result = Email::parse(email.clone());
        assert!(result.is_ok());
        assert_eq!(*result.unwrap().as_ref().expose_secret(), email);
    }

    #[test]
    fn test_invalid_email() {
        let email = "invalid-email".to_string();
        let result = Email::parse(email);
        assert!(matches!(result, Err(ParseError::InvalidEmail)));
    }
}
