#[derive(Debug, PartialEq, Clone, thiserror::Error)]
pub enum PasswordError {
    #[error("Password must be at least 8 characters long")]
    TooShort,
    #[error("Password must be at most 100 characters long")]
    TooLong,
    #[error("Password must contain at least one number")]
    NoNumber,
    #[error("Password must contain at least one special character")]
    NoSpecialCharacter,
}
#[derive(sqlx::Type)]
#[sqlx(transparent)]
#[derive(sqlx::FromRow, Debug, Clone, PartialEq)]
pub struct Password(String);

impl Password {
    pub fn parse(password: String) -> Result<Self, PasswordError> {
        if password.len() < 8 {
            return Err(PasswordError::TooShort);
        }
        if password.len() > 100 {
            return Err(PasswordError::TooLong);
        }
        if !password.chars().any(|c| c.is_numeric()) {
            return Err(PasswordError::NoNumber);
        }
        if !password
            .chars()
            .any(|c| "!§@#$%^&*()_+-=[]{}|;:,.<>?".contains(c))
        {
            return Err(PasswordError::NoSpecialCharacter);
        }

        Ok(Password(password))
    }

    pub fn unwrap(password: &str) -> Self {
        Self::parse(password.to_string()).unwrap()
    }
}

impl AsRef<str> for Password {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_password() {
        let password = "password123!".to_string();
        let result = Password::parse(password.clone());
        assert!(result.is_ok());
        assert_eq!(result.unwrap().as_ref(), password);
    }

    #[test]
    fn test_too_short_password() {
        let result = Password::parse("short".to_string());
        assert_eq!(result.err().unwrap(), PasswordError::TooShort);
    }

    #[test]
    fn test_too_long_password() {
        let result = Password::parse("x".repeat(101));
        assert_eq!(result.err().unwrap(), PasswordError::TooLong);
    }

    #[test]
    fn test_password_without_number() {
        let result = Password::parse("password!".to_string());
        assert_eq!(result.err().unwrap(), PasswordError::NoNumber);
    }

    #[test]
    fn test_password_without_special_character() {
        let result = Password::parse("password123".to_string());
        assert_eq!(result.err().unwrap(), PasswordError::NoSpecialCharacter);
    }
}
