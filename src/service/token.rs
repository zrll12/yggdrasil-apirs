use std::sync::Arc;
use std::time::Duration;

use chrono::{DateTime, Utc};
use lazy_static::lazy_static;
use moka::future::Cache;

use crate::model::serialized::uuid::UuidNoChar;

lazy_static! {
    static ref TOKEN_CACHE: Cache<String, TokenInfo> = Cache::builder()
        .time_to_live(Duration::from_secs(60 * 60 * 24 * 14)) //available for 14 days
        .build();
}

#[derive(Clone)]
pub struct TokenInfo {
    pub client_token: String,
    pub user_id: String,
    pub issued_time: DateTime<Utc>,
    pub available: bool,
}

#[derive(Eq, PartialEq)]
pub enum TokenState {
    Valid,
    TemporallyInvalid,
    Invalid,
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
    
    invalidate_tokens(&user_id, 10).await;

    (access_token, client_token)
}

pub async fn get_token_info(access_token: &str) -> Option<TokenInfo> {
    TOKEN_CACHE.get(access_token).await
}

pub async fn check_token_state(access_token: &str, client_token: Option<String>) -> TokenState {
    let token_info = get_token_info(access_token).await;
    if token_info.is_none() {
        return TokenState::Invalid;
    }
    let token_info = token_info.unwrap();
    if client_token.is_some() && token_info.client_token != client_token.unwrap() {
        return TokenState::Invalid;
    }

    if !token_info.available {
        return TokenState::TemporallyInvalid;
    }

    let now = Utc::now();
    if now - token_info.issued_time > chrono::Duration::days(7) {
        return TokenState::TemporallyInvalid;
    }
    TokenState::Valid
}

pub async fn invalidate_token(token: &str) {
    TOKEN_CACHE.invalidate(token).await;
}

pub async fn invalidate_tokens(user_id: &str, keep_alive: u8) {
    let mut tokens = TOKEN_CACHE
        .iter()
        .filter(|(_, token_info)| token_info.user_id == user_id)
        .collect::<Vec<(Arc<String>, TokenInfo)>>();
    if tokens.len() <= keep_alive as usize { 
        return;
    }
    
    tokens.sort_by_key(|token| token.1.issued_time);
    let tokens = tokens.iter().map(|(token, _)| token.clone().to_string()).collect::<Vec<String>>();
    for token in &tokens[..tokens.len() - keep_alive as usize] {
        TOKEN_CACHE.invalidate(token).await;
    }
}
