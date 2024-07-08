use serde::{Deserialize, Serialize};
use crate::model::generated::user::Model;
use crate::model::serialized::properties::Properties;
use crate::model::serialized::uuid::UuidNoChar;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SerializedUser {
    pub id: UuidNoChar,
    pub properties: Vec<Properties>,
}

impl From<Model> for SerializedUser {
    fn from(value: Model) -> Self {
        let mut properties = vec![];
        if value.preferred_language.is_some() {
            properties.push(Properties {
                name: "preferred_language".to_string(),
                value: value.preferred_language.unwrap(),
                signature: None
            })
        }
        
        SerializedUser {
            id: UuidNoChar::from(value.id),
            properties,
        }
    }
}