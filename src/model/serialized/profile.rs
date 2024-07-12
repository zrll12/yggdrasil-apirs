use base64::Engine;
use serde::{Deserialize, Serialize};

use crate::{CORE_CONFIG, TEXTURE_CONFIG};
use crate::model::generated::profile::Model;
use crate::model::serialized::properties::Properties;
use crate::service::crypto::rsa_sign;

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

impl SerializedProfile {
    pub async fn sign(&mut self) {
        for property in self.properties.iter_mut() {
            property.signature = Some(rsa_sign(property.value.as_bytes()));
        }
    }
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
                    skin: value.skin_texture.as_ref().map(|url| TextureMeta { url: CORE_CONFIG.base_url.clone() + "/textures/" + url }),
                    cape: value.cape_texture.as_ref().map(|url| TextureMeta { url: CORE_CONFIG.base_url.clone() + "/textures/" + url }),
                }
            };
            let textures = serde_json::to_string(&textures).unwrap();
            properties.push(Properties {
                name: "textures".to_string(),
                value: base64::engine::general_purpose::STANDARD.encode(textures.as_bytes()),
                signature: None
            });
        }
        
        let upload = if TEXTURE_CONFIG.allow_skin && TEXTURE_CONFIG.allow_cape {
            "skin,cape"
        } else if TEXTURE_CONFIG.allow_skin {
            "skin"
        } else if TEXTURE_CONFIG.allow_cape {
            "cape"
        } else { 
            ""
        };
        
        if upload != "" {
            properties.push(Properties {
                name: "uploadableTextures".to_string(),
                value: upload.to_string(),
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