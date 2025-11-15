use crate::store::ConnId;
use dashmap::DashMap;
use std::sync::Arc;

pub type RoomMap = Arc<DashMap<String, Vec<ConnId>>>;

pub fn setup_room_map() -> RoomMap {
    RoomMap::new(DashMap::new())
}
