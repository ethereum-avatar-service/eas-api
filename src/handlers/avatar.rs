use std::sync::Arc;
use axum::extract::State;
use axum::Json;

use crate::extractors::ethereum_address::EthereumAddress;
use crate::response::avatar::AvatarInfoWithMetadataResponse;
use crate::response::error::AppResult;
use crate::services::avatar::AvatarService;
use crate::supported_networks::SupportedNetworks;

#[allow(clippy::missing_errors_doc)]
pub async fn get(State(avatar_service): State<Arc<AvatarService>>, EthereumAddress(address): EthereumAddress) -> AppResult<Json<AvatarInfoWithMetadataResponse>> {
    let response = avatar_service.get_info_with_metadata(&address, SupportedNetworks::all()).await?;

    Ok(Json(response))
}
