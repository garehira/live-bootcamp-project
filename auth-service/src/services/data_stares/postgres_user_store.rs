use argon2::{
    password_hash::SaltString, Algorithm, Argon2, Params, PasswordHash, PasswordHasher,
    PasswordVerifier, Version,
};
use sqlx::{query, PgPool};

use crate::domain::data_stores::{UserStore, UserStoreError};
use crate::domain::password::Password;
use crate::domain::user::{User, UserRow};
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
    #[tracing::instrument(name = "Adding user to PostgreSQL", skip_all)]
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        let hash = compute_password_hash(user.password_hash.as_ref())
            .await
            .map_err(|e| UserStoreError::UnexpectedError(e.into()))?;
        query!(
            "INSERT INTO users (email, password_hash, requires_2fa) VALUES ($1, $2, $3)",
            user.email.as_ref(),
            hash,
            user.requires_2fa
        )
        .execute(&self.pool)
        .await
        .map_err(|e| UserStoreError::UnexpectedError(e.into()))?;
        Ok(())
    }
    #[tracing::instrument(name = "Retrieving user from PostgreSQL", skip_all)]
    async fn get_user(&self, email: &Email) -> Result<User, UserStoreError> {
        let row = sqlx::query_as!(UserRow, "SELECT * FROM USERS WHERE email = $1", email)
            .fetch_one(&self.pool)
            .await
            .map_err(|_| UserStoreError::UserNotFound)?;
        //
        // let row: Result<crate::domain::user::User, Error> =
        //     sqlx::query_as("SELECT * FROM USERS WHERE email = $1")
        //         .bind(email.as_ref())
        //         .fetch_one(&self.pool)
        //         .await;

        // row.map_err(|_| UserStoreError::UserNotFound)
        User::try_from(row).map_err(|e| UserStoreError::UnexpectedError(e.into()))
    }
    #[tracing::instrument(name = "Validating user credentials in PostgreSQL", skip_all)]
    async fn validate_user(
        &self,
        email: &Email,
        password: &Password,
    ) -> Result<(), UserStoreError> {
        let user = self.get_user(email).await?;
        verify_password_hash(user.password_hash.as_ref(), password.as_ref())
            .await
            .map_err(|_| UserStoreError::InvalidPassword)
    }
}
#[tracing::instrument(name = "Verify password hash", skip_all)]
async fn verify_password_hash(
    expected_password_hash: &str,
    password_candidate: &str,
) -> color_eyre::eyre::Result<()> {
    let current_span: tracing::Span = tracing::Span::current();

    let exp_hash = expected_password_hash.to_owned();
    let cand = password_candidate.to_owned();
    tokio::task::spawn_blocking(move || {
        current_span.in_scope(|| {
            let exp_hash_str: PasswordHash<'_> = PasswordHash::new(&*exp_hash)?;

            Argon2::default()
                .verify_password(cand.as_bytes(), &exp_hash_str)
                .map_err(|e| e.into())
        })
    })
    .await?
}

type ErrorType = Box<dyn std::error::Error + Send + Sync>;

#[tracing::instrument(name = "Computing password hash", skip_all)]
async fn compute_password_hash(password: &str) -> color_eyre::eyre::Result<String> {
    let pwd = password.to_owned();
    let current_span: tracing::Span = tracing::Span::current();

    tokio::task::spawn_blocking(move || -> color_eyre::eyre::Result<String> {
        current_span.in_scope(|| {
            let salt: SaltString = SaltString::generate(&mut rand::thread_rng());
            let password_hash = Argon2::new(
                Algorithm::Argon2id,
                Version::V0x13,
                Params::new(15000, 2, 1, None)?,
            )
            .hash_password(pwd.as_bytes(), &salt)?
            .to_string();

            // Err(eyre!("oh no!"))
            Ok(password_hash)
            // Err(Box::new(std::io::Error::other("oh no!"))) // as Box<dyn Error + Send + Sync>)
        })
    })
    .await?
}
