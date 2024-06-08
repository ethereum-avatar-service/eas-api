use alloy::primitives::Address;

use crate::response::avatar::AvatarInfoWithMetadataResponse;
use crate::services::rpc;
use crate::supported_networks::SupportedNetworks;

#[allow(clippy::missing_errors_doc)]
#[allow(clippy::missing_panics_doc)]
pub async fn get_info_with_metadata(address: &Address, networks: impl IntoIterator<Item =SupportedNetworks>) -> eyre::Result<AvatarInfoWithMetadataResponse> {
    let mut response = AvatarInfoWithMetadataResponse::default();

    let networks: Vec<SupportedNetworks> = networks.into_iter().collect();

    if networks.contains(&SupportedNetworks::Sepolia) {
        let provider = rpc::sepolia::new();
        let maybe_avatar_info = provider.get_avatar_info_with_metadata(address).await.ok();

        response.networks.insert("sepolia".to_string(), maybe_avatar_info);
    }

    if networks.contains(&SupportedNetworks::Polygon) {
        let provider = rpc::polygon::new();
        let maybe_avatar_info = provider.get_avatar_info_with_metadata(address).await.ok();
        
        response.networks.insert("polygon".to_string(), maybe_avatar_info);
    }

    Ok(response)
}
