use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Properties {
    pub name: String,
    pub value: String,
    pub signature: Option<String>
}