use std::{collections::HashMap, sync::Arc};

use tokio::sync::Mutex;

use crate::store::ConnId;

pub type RoomMap = Arc<Mutex<HashMap<String, Vec<ConnId>>>>;

pub fn setup_room_map() -> RoomMap {
    RoomMap::new(Mutex::new(HashMap::new()))
}
