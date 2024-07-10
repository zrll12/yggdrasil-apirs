use lazy_static::lazy_static;
use moka::future::Cache;

lazy_static! {
    pub static ref SESSION_CACHE: Cache<String, SessionInfo> = Cache::builder()
        .time_to_live(std::time::Duration::from_secs(30)) //available for 30 seconds
        .build();
}

#[derive(Clone, Debug)]
pub struct SessionInfo {
    pub access_token: String,
    pub client_ip: String,
}

pub async fn save_session(server_id: String, info: SessionInfo) {
    SESSION_CACHE.insert(server_id, info).await;
}