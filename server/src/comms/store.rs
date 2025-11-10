use serde::{Deserialize, Serialize};
use std::sync::Arc;

use axum::extract::ws::Message;
use chrono::{DateTime, Utc};
use dashmap::DashMap;
use once_cell::sync::Lazy;
use tokio::sync::mpsc::Sender;
use uuid::Uuid;
use webrtc::peer_connection::RTCPeerConnection;

pub type ConnId = Uuid;
pub type RoomId = String;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConnMetaData {
    pub user_id: String,
    pub room_id: RoomId,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub last_verified_at: DateTime<Utc>,
}

pub static SOCKET_CONNS: Lazy<DashMap<ConnId, Sender<Message>>> = Lazy::new(|| DashMap::new());

pub static SFU_PEERS: Lazy<DashMap<ConnId, Arc<RTCPeerConnection>>> =
    Lazy::new(|| DashMap::new());

