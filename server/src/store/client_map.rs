use dashmap::DashMap;
use std::sync::Arc;
use tokio::sync::mpsc::Sender;
use uuid::Uuid;

pub type ConnId = Uuid;

pub type ClientMap = Arc<DashMap<ConnId, Sender<String>>>;

pub fn setup_client_map() -> ClientMap {
    ClientMap::new(DashMap::new())
}
