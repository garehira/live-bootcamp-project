use argon2::{
    password_hash::SaltString, Algorithm, Argon2, Params, PasswordHash, PasswordHasher,
    PasswordVerifier, Version,
};
use sqlx::{query, Error, PgPool};

use crate::domain::data_stores::{UserStore, UserStoreError};
use crate::domain::password::Password;
use crate::domain::user::User;
use crate::domain::Email;

pub struct PostgresUserStore {
    pool: PgPool,
}

impl PostgresUserStore {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl UserStore for PostgresUserStore {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        let hash = compute_password_hash(user.password.as_ref())
            .await
            .map_err(|e| UserStoreError::InvalidCredentials)?;
        query!(
            "INSERT INTO users (email, password_hash, requires_2fa) VALUES ($1, $2, $3)",
            user.email.as_ref(),
            hash,
            user.requires_2fa
        )
        .execute(&self.pool)
        .await
        .map_err(|e| UserStoreError::UserAlreadyExists)?;
        Ok(())
    }

    async fn get_user(&self, email: &Email) -> Result<User, UserStoreError> {
        let row: Result<crate::domain::user::User, Error> =
            sqlx::query_as("SELECT * FROM USERS WHERE email = $1")
                .bind(email.as_ref())
                .fetch_one(&self.pool)
                .await;

        row.map_err(|e| UserStoreError::UserNotFound)
    }

    async fn validate_user(
        &self,
        email: &Email,
        password: &Password,
    ) -> Result<(), UserStoreError> {
        let user = self.get_user(email).await?;
        verify_password_hash(user.password.as_ref(), password.as_ref())
            .await
            .map_err(|e| UserStoreError::InvalidPassword)
    }
}

async fn verify_password_hash(
    expected_password_hash: &str,
    password_candidate: &str,
) -> Result<(), ErrorType> {
    let exp_hash = expected_password_hash.to_owned();
    let cand = password_candidate.to_owned();
    tokio::task::spawn_blocking(move || -> Result<(), ErrorType> {
        let exp_hash_str: PasswordHash<'_> = PasswordHash::new(&*exp_hash)?;

        Argon2::default()
            .verify_password(cand.as_bytes(), &exp_hash_str)
            .map_err(|e| e.into())
    })
    .await?
}

type ErrorType = Box<dyn std::error::Error + Send + Sync>;
async fn compute_password_hash(password: &str) -> Result<String, ErrorType> {
    let pwd = password.to_owned();
    tokio::task::spawn_blocking(move || -> Result<String, ErrorType> {
        let salt: SaltString = SaltString::generate(&mut rand::thread_rng());
        let password_hash = Argon2::new(
            Algorithm::Argon2id,
            Version::V0x13,
            Params::new(15000, 2, 1, None)?,
        )
        .hash_password(pwd.as_bytes(), &salt)?
        .to_string();

        Ok(password_hash)
    })
    .await?
}
