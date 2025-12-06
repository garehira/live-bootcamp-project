use color_eyre::eyre::Result;
use secrecy::{ExposeSecret, Secret};

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

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct Password(Secret<String>);

impl PartialEq for Password {
    // New!
    fn eq(&self, other: &Self) -> bool {
        // We can use the expose_secret method to expose the secret in a
        // controlled manner when needed!
        self.0.expose_secret() == other.0.expose_secret() // Updated!
    }
}

impl Password {
    pub fn parse(password: Secret<String>) -> Result<Self, PasswordError> {
        if password.expose_secret().len() < 8 {
            return Err(PasswordError::TooShort);
        }
        if password.expose_secret().len() > 100 {
            return Err(PasswordError::TooLong);
        }
        if !password.expose_secret().chars().any(|c| c.is_numeric()) {
            return Err(PasswordError::NoNumber);
        }
        if !password
            .expose_secret()
            .chars()
            .any(|c| "!§@#$%^&*()_+-=[]{}|;:,.<>?".contains(c))
        {
            return Err(PasswordError::NoSpecialCharacter);
        }

        Ok(Password(password))
    }
    //
    // pub fn unwrap(password: &str) -> Self {
    //     Self::parse(password.to_string()).unwrap()
    // }
}
impl AsRef<Secret<String>> for Password {
    // Updated!
    fn as_ref(&self) -> &Secret<String> {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_string_is_rejected() {
        let password = Secret::new("".to_string()); // Updated!
        assert!(Password::parse(password).is_err());
    }
    #[test]
    fn string_less_than_8_characters_is_rejected() {
        let password = Secret::new("1234567".to_string()); // Updated!
        assert!(Password::parse(password).is_err());
    }

    #[test]
    fn test_valid_password() {
        let password = Secret::new("password123!".to_string());
        let result = Password::parse(password.clone());
        assert!(result.is_ok());
        assert_eq!(
            result.as_ref().unwrap().0.expose_secret(),
            password.expose_secret()
        );
    }

    #[test]
    fn test_too_short_password() {
        let result = Password::parse(Secret::new("short".to_string()));
        assert_eq!(result.err().unwrap(), PasswordError::TooShort);
    }

    #[test]
    fn test_too_long_password() {
        let result = Password::parse(Secret::new("x".repeat(101)));
        assert_eq!(result.err().unwrap(), PasswordError::TooLong);
    }

    #[test]
    fn test_password_without_number() {
        let result = Password::parse(Secret::new("password!".to_string()));
        assert_eq!(result.err().unwrap(), PasswordError::NoNumber);
    }

    #[test]
    fn test_password_without_special_character() {
        let result = Password::parse(Secret::new("password123".to_string()));
        assert_eq!(result.err().unwrap(), PasswordError::NoSpecialCharacter);
    }

    // #[derive(Debug, Clone)]
    // struct ValidPasswordFixture(pub Secret<String>); // Updated!

    // impl quickcheck::Arbitrary for ValidPasswordFixture {
    //     fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> Self {
    //         let password = FakePassword(8..30).fake_with_rng(g);
    //         Self(Secret::new(password)) // Updated!
    //     }
    // }
    // #[quickcheck_macros::quickcheck]
    // fn valid_passwords_are_parsed_successfully(valid_password: ValidPasswordFixture) -> bool {
    //     Password::parse(valid_password.0).is_ok()
    // }
}
