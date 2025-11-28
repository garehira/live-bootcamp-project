use crate::app_state::AppState;
use crate::routes::{login, logout, signup, verify_2fa, verify_token};
use http::Method;

use crate::util::tracing::{make_span_with_request_id, on_request, on_response};
use axum::routing::post;
use axum::serve::Serve;
use axum::Router;
use http;
use redis::{Client, RedisResult};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::error::Error;
use std::sync::Arc;
use tower_http::trace::TraceLayer;
use tower_http::{cors::CorsLayer, services::ServeDir};

pub mod app_state;
pub mod domain;
pub mod routes;
pub mod services;
pub mod util;

// This struct encapsulates our application-related logic.
pub struct Application {
    server: Serve<Router, Router>,
    // address is exposed as a public field
    // so we have access to it in tests.
    pub address: String,
}

impl Application {
    pub async fn build(app_state: AppState, address: &str) -> Result<Self, Box<dyn Error>> {
        let allowed_origins = [
            "http://localhost:8000".parse()?,
            // TODO: Replace [YOUR_DROPLET_IP] with your Droplet IP address
            // "http://[YOUR_DROPLET_IP]:8000".parse()?,
        ];

        let cors = CorsLayer::new()
            // Allow GET and POST requests
            //     .allow_methods([axum::http::Method::GET, axum::http::Method::POST])
            .allow_methods([Method::GET, Method::POST])
            //     .allow_methods::<AllowMethods>(reqwest::Method::GET.into())
            // Allow cookies to be included in requests
            .allow_credentials(true)
            .allow_origin(allowed_origins);

        let router = Router::new()
            .nest_service("/", ServeDir::new("assets"))
            // .route("/", get(login))
            .route("/signup", post(signup))
            .route("/login", post(login))
            .route("/verify-2fa", post(verify_2fa))
            .route("/logout", post(logout))
            .route("/verify-token", post(verify_token))
            .with_state(Arc::new(app_state))
            .layer(cors)
            .layer(
                TraceLayer::new_for_http()
                    .make_span_with(make_span_with_request_id)
                    .on_request(on_request)
                    .on_response(on_response),
            );

        let listener = tokio::net::TcpListener::bind(address).await?;
        let address = listener.local_addr()?.to_string();
        let server = axum::serve(listener, router);

        // Create a new Application instance and return it
        Ok(Application { server, address })
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        tracing::info!("listening on {}", &self.address);
        self.server.await
    }
}

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}
pub async fn get_postgres_pool(url: &str) -> Result<PgPool, sqlx::Error> {
    // Create a new PostgreSQL connection pool
    PgPoolOptions::new().max_connections(5).connect(url).await
}

pub fn get_redis_client(redis_hostname: String) -> RedisResult<Client> {
    let redis_url = format!("redis://{}/", redis_hostname);
    redis::Client::open(redis_url)
}
