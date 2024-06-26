use alloy::primitives::{Address, U256};
use serde::{Serialize, Serializer};

use crate::services;

#[derive(Serialize)]
pub struct Avatar {
    pub token_address: Address,
    #[serde(serialize_with = "serialize_u256_as_decimal")]
    pub token_id: U256
}

impl From<services::rpc::AvatarService::Avatar> for Avatar {
    fn from(value: services::rpc::AvatarService::Avatar) -> Self {
        Self {
            token_address: value.tokenAddress,
            token_id: value.tokenId,
        }
    }
}

fn serialize_u256_as_decimal<S>(value: &U256, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
{
    serializer.serialize_str(&value.to_string())
}

#[allow(clippy::module_name_repetitions)]
#[derive(Serialize)]
pub struct AvatarInfo {
    pub avatar: Avatar,
    pub owned: bool,
    pub uri: String
}

impl From<services::rpc::AvatarService::AvatarInfo> for AvatarInfo {
    fn from(value: services::rpc::AvatarService::AvatarInfo) -> Self {
        Self {
            avatar: Avatar::from(value.avatar),
            owned: value.owned,
            uri: value.uri,
        }
    }
}

#[allow(clippy::module_name_repetitions)]
#[derive(Default, Serialize, Clone)]
pub struct AvatarCollection {
    pub name: Option<String>,
    pub author: Option<String>,
    pub website: Option<String>,
    pub opensea: Option<String>,
    pub verified: bool
}

#[allow(clippy::module_name_repetitions)]
#[derive(Default, Serialize)]
pub struct AvatarMetadata {
    pub image: Option<String>,
    pub collection: Option<AvatarCollection>
}

#[allow(clippy::module_name_repetitions)]
#[derive(Serialize)]
pub struct AvatarInfoWithMetadata {
    pub avatar: Avatar,
    pub owned: bool,
    pub uri: String,
    pub avatar_metadata: AvatarMetadata
}

#[allow(clippy::module_name_repetitions)]
#[derive(Serialize, Debug, Eq, PartialEq, Hash)]
pub enum AvatarType {
    #[serde(rename = "flat")]
    Flat,
    #[serde(rename = "composite")]
    Composite
}