use std::time::Duration;
use chrono::{DateTime, Utc};
use lazy_static::lazy_static;
use moka::future::Cache;
use crate::model::serialized::uuid::UuidNoChar;

lazy_static! {
    static ref TOKEN_CACHE: Cache<String, TokenInfo> = Cache::builder()
        .time_to_live(Duration::from_secs(60 * 60 * 24 * 7)) //available for 7 days
        .build();
}

#[derive(Clone)]
pub struct TokenInfo {
    pub client_token: String,
    pub user_id: String,
    pub issued_time: DateTime<Utc>,
    pub available: bool,
}

fn new_token() -> String {
    UuidNoChar::new().to_string()
}

pub async fn sign_new_token(user_id: String, client_token: Option<String>) -> (String, String) {
    let access_token = new_token();
    let client_token = client_token.unwrap_or_else(|| new_token());
    let token_info = TokenInfo {
        client_token: client_token.clone(),
        user_id: user_id.clone(),
        issued_time: Utc::now(),
        available: true,
    };
    TOKEN_CACHE.insert(access_token.clone(), token_info).await;
    (access_token, client_token)
}

