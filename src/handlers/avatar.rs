use std::sync::Arc;
use axum::extract::{Query, State};
use axum::Json;
use serde::Deserialize;

use crate::extractors::ethereum_address::EthereumAddress;
use crate::response::avatar::AvatarInfoWithMetadataResponse;
use crate::response::error::AppResult;
use crate::services::avatar::AvatarService;
use crate::supported_networks::SupportedNetworks;

#[derive(Deserialize)]
pub struct GetParams {
    metadata: Option<bool>,
    network: Option<SupportedNetworks>
}

#[allow(clippy::missing_errors_doc)]
pub async fn get(State(avatar_service): State<Arc<AvatarService>>, EthereumAddress(address): EthereumAddress, Query(params): Query<GetParams>) -> AppResult<Json<AvatarInfoWithMetadataResponse>> {
    let networks = if let Some(network) = params.network {
        vec![network]
    } else {
        SupportedNetworks::all()
    };
    
    let response = avatar_service.get_info_with_metadata(&address, networks).await?;

    Ok(Json(response))
}
