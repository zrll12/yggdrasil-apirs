use serde::{Deserialize, Serialize};
use serde_inline_default::serde_inline_default;

#[serde_inline_default]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TextureConfig {
    #[serde_inline_default(true)]
    pub allow_skin: bool,
    #[serde_inline_default(true)]
    pub allow_cape: bool,
    #[serde_inline_default(256)]
    pub max_width: u32,
    #[serde_inline_default(256)]
    pub max_height: u32,
    #[serde_inline_default(vec![String::from("127.0.0.1:7890")])]
    pub skin_domains: Vec<String>
}