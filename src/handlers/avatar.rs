use axum::Json;

use crate::extractors::ethereum_address::EthereumAddress;
use crate::response::avatar::AvatarInfoWithMetadataResponse;
use crate::response::error::AppResult;
use crate::services;
use crate::supported_networks::SupportedNetworks;

#[allow(clippy::missing_errors_doc)]
pub async fn get(EthereumAddress(address): EthereumAddress, ) -> AppResult<Json<AvatarInfoWithMetadataResponse>> {
    let response = services::avatar::get_info_with_metadata(&address, SupportedNetworks::all()).await?;

    Ok(Json(response))
}
