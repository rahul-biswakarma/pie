use crate::store::ConnId;
use dashmap::DashMap;
use std::sync::Arc;

#[derive(Clone, Default)]
pub struct WsMetadata {
    pub user_id: String,
    pub room_id: Option<String>,
}

pub type ClientMetadata = Arc<DashMap<ConnId, WsMetadata>>;

pub fn setup_client_metadata_map() -> ClientMetadata {
    Arc::new(DashMap::new())
}
