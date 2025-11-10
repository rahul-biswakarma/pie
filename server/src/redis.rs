extern crate redis;
use super::comms::store::{ConnId, ConnMetaData, RoomId};
use anyhow::Result;
use redis::{Commands, Connection};
use std::env;
use std::sync::Arc; // Arc is from std
use tokio::sync::Mutex; // Mutex is from tokio

pub fn create_redis_connection() -> redis::Connection {
    let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set");
    let client = redis::Client::open(redis_url.as_str());
    client
        .expect("REDIS URL must be valid")
        .get_connection()
        .expect("REDIS connection must be valid")
}

fn get_room_key(room_id: &RoomId) -> String {
    format!("room:{}", room_id)
}

fn get_conn_key(conn_id: &ConnId) -> String {
    format!("conn:{}", conn_id)
}

pub async fn add_to_room(
    redis: Arc<Mutex<Connection>>,
    room_id: &RoomId,
    conn_id: &ConnId,
) -> Result<()> {
    let mut redis = redis.lock().await;
    let room_key = get_room_key(room_id);
    Ok(redis.sadd(room_key, conn_id.to_string())?)
}

pub async fn remove_from_room(
    redis: Arc<Mutex<Connection>>,
    room_id: &RoomId,
    conn_id: &ConnId,
) -> Result<()> {
    let mut redis = redis.lock().await;
    let room_key = get_room_key(room_id);
    Ok(redis.srem(room_key, conn_id.to_string())?)
}

pub async fn get_room_members(
    redis: Arc<Mutex<Connection>>,
    room_id: &RoomId,
) -> Result<Vec<String>> {
    let mut redis = redis.lock().await;
    let room_key = get_room_key(room_id);
    Ok(redis.smembers(room_key)?)
}

pub async fn set_conn_metadata(
    redis: Arc<Mutex<Connection>>,
    conn_id: &ConnId,
    metadata: &ConnMetaData,
) -> Result<()> {
    let mut redis = redis.lock().await;
    let conn_key = get_conn_key(conn_id);
    let metadata_str = serde_json::to_string(metadata)?;
    Ok(redis.set(conn_key, metadata_str)?)
}

pub async fn get_conn_metadata(
    redis: Arc<Mutex<Connection>>,
    conn_id: &ConnId,
) -> Result<Option<ConnMetaData>> {
    let mut redis = redis.lock().await;
    let conn_key = get_conn_key(conn_id);
    let metadata_str: Option<String> = redis.get(conn_key)?;
    match metadata_str {
        Some(s) => Ok(Some(serde_json::from_str(&s)?)),
        None => Ok(None),
    }
}

pub async fn remove_conn_metadata(redis: Arc<Mutex<Connection>>, conn_id: &ConnId) -> Result<()> {
    let mut redis = redis.lock().await;
    let conn_key = get_conn_key(conn_id);
    Ok(redis.del(conn_key)?)
}

pub async fn remove_connection(redis: Arc<Mutex<Connection>>, conn_id: &ConnId) -> Result<()> {
    if let Some(metadata) = get_conn_metadata(redis.clone(), conn_id).await? {
        remove_from_room(redis.clone(), &metadata.room_id, conn_id).await?;
    }
    remove_conn_metadata(redis, conn_id).await?;
    Ok(())
}

