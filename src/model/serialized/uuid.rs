use std::fmt::Display;
use serde::{Deserialize, Deserializer, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct UuidNoChar {
    pub uuid: Uuid,
}

impl Serialize for UuidNoChar {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.uuid.to_string().replace("-", ""))
    }
}

impl<'de> Deserialize<'de> for UuidNoChar {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let uuid = String::deserialize(deserializer)?;
        Ok(UuidNoChar {
            uuid: Uuid::parse_str(&uuid).unwrap(),
        })
    }
}

impl From<String> for UuidNoChar {
    fn from(value: String) -> Self {
        UuidNoChar {
            uuid: Uuid::parse_str(&value).unwrap(),
        }
    }
}

impl From<Uuid> for UuidNoChar {
    fn from(value: Uuid) -> Self {
        UuidNoChar { uuid: value }
    }
}

impl Display for UuidNoChar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.uuid.to_string().replace("-", ""))
    }
}