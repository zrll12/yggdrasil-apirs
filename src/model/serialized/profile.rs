use base64::Engine;
use serde::{Deserialize, Serialize};
use crate::config::core::get_server_base_url;
use crate::model::generated::profile::Model;
use crate::model::serialized::properties::Properties;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SerializedProfile {
    pub id: String,
    pub name: String,
    pub properties: Vec<Properties>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Textures {
    pub timestamp: i64,
    pub profile_id: String,
    pub profile_name: String,
    pub textures: TexturesData
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TexturesData {
    #[serde(rename = "SKIN")]
    pub skin: Option<TextureMeta>,
    #[serde(rename = "CAPE")]
    pub cape: Option<TextureMeta>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TextureMeta {
    url: String,
}

impl From<Model> for SerializedProfile {
    fn from(value: Model) -> Self {
        let mut properties = vec![];
        if value.skin_texture.is_some() || value.cape_texture.is_some() { 
            let textures = Textures {
                timestamp: value.create_time.and_utc().timestamp_millis(),
                profile_id: value.id.clone(),
                profile_name: value.name.clone(),
                textures: TexturesData {
                    skin: value.skin_texture.as_ref().map(|url| TextureMeta { url: get_server_base_url() + "/textures/" + url }),
                    cape: value.cape_texture.as_ref().map(|url| TextureMeta { url: get_server_base_url() + "/textures/" + url }),
                }
            };
            let textures = serde_json::to_string(&textures).unwrap();
            properties.push(Properties {
                name: "textures".to_string(),
                value: base64::engine::general_purpose::STANDARD.encode(textures.as_bytes()),
                signature: None
            });
        }
        
        SerializedProfile {
            id: value.id,
            name: value.name,
            properties,
        }
    }
}