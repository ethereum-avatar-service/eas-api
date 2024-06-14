use std::collections::HashMap;
use serde::Serialize;

use crate::models::avatar::{AvatarInfo, AvatarInfoWithMetadata, AvatarType};

#[derive(Default, Serialize)]
pub struct AvatarInfoResponse {
    pub networks: HashMap<String, HashMap<AvatarType, Option<AvatarInfo>>>
}

#[derive(Default, Serialize)]
pub struct AvatarInfoWithMetadataResponse {
    pub networks: HashMap<String, HashMap<AvatarType, Option<AvatarInfoWithMetadata>>>
}
