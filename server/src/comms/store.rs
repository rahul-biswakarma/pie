use std::sync::Arc;

use axum::extract::ws::Message;
use chrono::{DateTime, Utc};
use dashmap::{DashMap, DashSet};
use once_cell::sync::Lazy;
use tokio::sync::mpsc::Sender;
use uuid::Uuid;
use webrtc::peer_connection::RTCPeerConnection;

pub type ConnId = Uuid;
pub type RoomId = String;

#[derive(Default)]
pub struct ConnMetaData {
    pub user_id: String,
    pub room_id: RoomId,
    pub last_verified_at: DateTime<Utc>,
}

pub static SOCKET_CONNS: Lazy<DashMap<ConnId, Sender<Message>>> = Lazy::new(|| DashMap::new());

pub static ROOMS: Lazy<DashMap<RoomId, DashSet<ConnId>>> = Lazy::new(|| DashMap::new());

pub static CONN_METADATA: Lazy<DashMap<ConnId, ConnMetaData>> = Lazy::new(|| DashMap::new());

pub static SFU_PEERS: Lazy<DashMap<ConnId, Arc<RTCPeerConnection>>> = Lazy::new(|| DashMap::new());
