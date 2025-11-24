use crate::domain::email::{Email, ParseError};
use crate::domain::password::{Password, PasswordError};

// The User struct should contain 3 fields. email, which is a String;
// password, which is also a String; and requires_2fa, which is a boolean.
#[derive(sqlx::FromRow, Debug, PartialEq, Clone)]
pub struct User {
    pub email: Email,
    pub password: Password,
    pub requires_2fa: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub enum UserError {
    EmailError(ParseError),
    PasswordError(PasswordError),
}

impl From<ParseError> for UserError {
    fn from(error: ParseError) -> Self {
        UserError::EmailError(error)
    }
}
impl From<PasswordError> for UserError {
    fn from(error: PasswordError) -> Self {
        UserError::PasswordError(error)
    }
}

impl User {
    pub fn new(email: &str, password: &str, requires_2fa: bool) -> Result<Self, UserError> {
        Ok(User {
            email: Email::parse(email.to_string())?,
            password: Password::parse(password.to_string())?,
            requires_2fa,
        })
    }
    pub fn new2(email: Email, password: Password, requires_2fa: bool) -> Self {
        User {
            email,
            password,
            requires_2fa,
        }
    }
}
