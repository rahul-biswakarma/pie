mod error;
mod store;
mod webscoket;

use std::env;

use axum::routing::any;
use tokio::net::TcpListener;

use crate::{
    store::{setup_client_map, setup_client_metadata_map, setup_room_map, ClientMap, ConnId},
    webscoket::{handle_text_message, handle_ws_upgrade},
};

#[derive(Clone)]
struct AppState {
    tx_inbound: tokio::sync::mpsc::Sender<(ConnId, String)>,
    client_map: ClientMap,
    jwt_secret: String,
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let room_map = setup_room_map();
    let client_map = setup_client_map();
    let metadata_map = setup_client_metadata_map();

    let jwt_secret = env::var("SUPABASE_JWT_SECRET").expect("SUPABASE_JWT_SECRET must be set");

    let listner = TcpListener::bind("127.0.0.1:3001")
        .await
        .expect("Could not bind to 127.0.0.1:3001");

    let (tx_inbound, mut rx_inbound) = tokio::sync::mpsc::channel::<(ConnId, String)>(156);

    let state = AppState {
        tx_inbound,
        client_map: client_map.clone(),
        jwt_secret,
    };

    let app = axum::Router::new()
        .route("/socket", any(handle_ws_upgrade))
        .with_state(state);

    tokio::spawn(async move {
        while let Some((conn_id, message)) = rx_inbound.recv().await {
            handle_text_message(
                conn_id,
                message.to_string(),
                client_map.clone(),
                room_map.clone(),
                metadata_map.clone(),
            )
            .await;
        }
    });

    axum::serve(listner, app).await.unwrap();
}
