use crate::store::ConnId;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

pub struct WsMetadata {
    pub user_id: String,
}

pub type ClientMetadata = Arc<Mutex<HashMap<ConnId, WsMetadata>>>;

pub fn setup_client_metadata_map() -> ClientMetadata {
    Arc::new(Mutex::new(HashMap::new()))
}
