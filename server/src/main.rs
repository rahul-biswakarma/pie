mod auth;
mod comms;
mod error;

use axum::routing::get;
use comms::websocket_handler;
use dashmap::DashMap;
use std::{env, sync::Arc};
use tracing_subscriber;

#[derive(Clone)]
pub struct AppState {
    pub jwks_url: String,
    pub jwks_cache: Arc<DashMap<String, Jwk>>,
    pub supabase_anon_key: String,
    pub jwt_secret: String,
}

#[derive(Clone, Debug)]
pub struct Jwk {
    pub kid: String,
    pub n: String,
    pub e: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    dotenvy::dotenv().ok();

    let supabase_url = env::var("SUPABASE_URL").expect("SUPABASE_URL must be set");

    let jwks_url = format!("{}/auth/v1/.well-known/jwks.json", supabase_url.trim_end_matches('/'));

    let supabase_anon_key = env::var("SUPABASE_ANON_KEY").expect("SUPABASE_ANON_KEY must be set");
    
    let jwt_secret = env::var("SUPABASE_JWT_SECRET").expect("SUPABASE_JWT_SECRET must be set");

    tracing::info!("Starting WebSocket server on 127.0.0.1:3001");
    tracing::info!("JWKS URL: {}", jwks_url);

    let state = AppState {
        jwks_url,
        jwks_cache: Arc::new(DashMap::new()),
        supabase_anon_key,
        jwt_secret,
    };

    let route = axum::Router::new()
        .route("/socket", get(websocket_handler))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .expect("Failed to bind to 127.0.0.1:3001");

    tracing::info!("WebSocket server listening on 127.0.0.1:3001");

    axum::serve(listener, route).await.expect("Server failed");
}
