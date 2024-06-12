use std::collections::HashMap;
use std::sync::Arc;

use alloy::primitives::Address;
use axum::extract::State;
use axum::Json;

use crate::models::avatar::AvatarCollection;
use crate::response::error::AppResult;
use crate::services::avatar::AvatarService;
use crate::supported_networks::SupportedNetworks;

#[allow(clippy::missing_errors_doc)]
pub async fn get(State(avatar_service): State<Arc<AvatarService>>) -> AppResult<Json<HashMap<SupportedNetworks, HashMap<Address, AvatarCollection>>>> {
    let response = avatar_service.cache.verified_collections.read().await.clone();

    Ok(Json(response))
}