use std::{collections::HashMap, sync::Arc};
use tokio::sync::{mpsc::Sender, Mutex};
use uuid::Uuid;

pub type ConnId = Uuid;

pub type ClientMap = Arc<Mutex<HashMap<ConnId, Sender<String>>>>;

pub fn setup_client_map() -> ClientMap {
    ClientMap::new(Mutex::new(HashMap::new()))
}
