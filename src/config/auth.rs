use serde::{Deserialize, Serialize};
use serde_inline_default::serde_inline_default;

#[serde_inline_default]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AuthConfig {
    #[serde_inline_default(7)]
    pub token_valid_days: i64,
    #[serde_inline_default(14)]
    pub token_keep_days: i64,
    #[serde_inline_default(10)]
    pub login_rate_limit: u32,
    #[serde_inline_default(10)]
    pub max_token_allowed: u32,
}