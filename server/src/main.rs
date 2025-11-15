mod store;
mod webscoket;

use std::env;

use crate::{
    store::{
        setup_client_map, setup_client_metadata_map, setup_room_map, ClientMap, ClientMetadata,
        RoomMap,
    },
    webscoket::handle_ws_upgrade,
};
use axum::routing::any;
use tokio::net::TcpListener;

#[derive(Clone)]
pub struct AppState {
    client_map: ClientMap,
    metadata_map: ClientMetadata,
    room_map: RoomMap,
    jwt_secret: String,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let state = AppState {
        client_map: setup_client_map(),
        metadata_map: setup_client_metadata_map(),
        room_map: setup_room_map(),
        jwt_secret: env::var("SUPABASE_JWT_SECRET").expect("SUPABASE_JWT_SECRET must be set"),
    };

    let app = axum::Router::new()
        .route("/socket", any(handle_ws_upgrade))
        .with_state(state);

    let listner = TcpListener::bind("127.0.0.1:3001")
        .await
        .expect("Could not bind to 127.0.0.1:3001");

    tracing::info!("listening on http://127.0.0.1:3001");
    axum::serve(listner, app).await.unwrap();
}
