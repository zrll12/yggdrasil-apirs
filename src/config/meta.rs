use serde::{Deserialize, Serialize};
use serde_inline_default::serde_inline_default;

#[serde_inline_default]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MetaConfig {
    #[serde_inline_default(String::from("rust api"))]
    pub server_name: String,
    #[serde_inline_default(FeatureConfig::new())]
    pub feature: FeatureConfig,
}

#[serde_inline_default]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FeatureConfig {
    #[serde_inline_default(true)]
    pub non_email_login: bool,
}

impl FeatureConfig {
    fn new() -> Self {
        FeatureConfig {
            non_email_login: true,
        }
    }
}