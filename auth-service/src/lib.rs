use crate::app_state::AppState;
use crate::routes::{login, logout, signup, verify_2fa, verify_token};
use http::Method;

use axum::routing::post;
use axum::serve::Serve;
use axum::Router;
use http;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::sync::Arc;
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

        let _cors = CorsLayer::new()
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
            .with_state(Arc::new(app_state));

        let listener = tokio::net::TcpListener::bind(address).await?;
        let address = listener.local_addr()?.to_string();
        let server = axum::serve(listener, router);

        // Create a new Application instance and return it
        Ok(Application { server, address })
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        println!("listening on {}", &self.address);
        self.server.await
    }
}

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}
