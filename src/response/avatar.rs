use std::collections::HashMap;
use serde::Serialize;

use crate::models::avatar::{AvatarInfo, AvatarInfoWithMetadata};

#[derive(Default, Serialize)]
pub struct AvatarInfoResponse {
    pub networks: HashMap<String, Option<AvatarInfo>>
}

#[derive(Default, Serialize)]
pub struct AvatarInfoWithMetadataResponse {
    pub networks: HashMap<String, Option<AvatarInfoWithMetadata>>
}
