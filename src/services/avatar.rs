use std::collections::HashMap;
use std::sync::Arc;

use alloy::primitives::{Address, U256};
use tokio::sync::RwLock;

use crate::models::avatar::{AvatarCollection, AvatarType};
use crate::models::nft::NftMetadata;
use crate::models::whitelist;
use crate::response::avatar::AvatarInfoWithMetadataResponse;
use crate::services::rpc;
use crate::supported_networks::SupportedNetworks;

pub type VerifiedCollections = HashMap<SupportedNetworks, HashMap<Address, AvatarCollection>>;
pub type IpfsCache = HashMap<String, NftMetadata>;
pub type TokenUriCache = HashMap<SupportedNetworks, HashMap<(Address, U256), String>>;

#[allow(clippy::module_name_repetitions)]
#[derive(Default)]
pub struct AvatarServiceCache {
    pub verified_collections: RwLock<VerifiedCollections>,
    pub ipfs: Arc<RwLock<IpfsCache>>,
    pub token_uris: Arc<RwLock<TokenUriCache>>
}

#[allow(clippy::module_name_repetitions)]
#[derive(Default)]
pub struct AvatarService {
    pub cache: Arc<AvatarServiceCache>
}

impl AvatarService {
    pub async fn reload_verified_collections(&self) {
        let result = reqwest::get("https://raw.githubusercontent.com/ethereum-avatar-service/eas-api-whitelist/main/collections.json").await;

        let mut verified_collections = self.cache.verified_collections.write().await;

        if let Ok(response) = result {
            match response.json::<whitelist::Collections>().await {
                Ok(collections) => {
                    for (network, network_collections) in collections.0 {
                        for collection in network_collections {
                            if let Ok(address) = collection.contract.parse::<Address>() {
                                let chain = match network.to_lowercase().as_str() {
                                    "mainnet" | "ethereum" => SupportedNetworks::Ethereum,
                                    "sepolia" => SupportedNetworks::Sepolia,
                                    "polygon" => SupportedNetworks::Polygon,
                                    "base" => SupportedNetworks::Base,
                                    _ => { continue; }
                                };

                                let entry = verified_collections.entry(chain).or_default();

                                entry.insert(address, AvatarCollection {
                                    name: Some(collection.name.clone()),
                                    author: Some(collection.author.clone()),
                                    website: Some(collection.website.clone()),
                                    opensea: collection.opensea.clone(),
                                    verified: true,
                                });
                            }
                        }
                    }
                }
                Err(err) => {
                    println!("{err}");
                }
            }
        }
    }

    #[allow(clippy::missing_errors_doc)]
    #[allow(clippy::missing_panics_doc)]
    pub async fn get_info_with_metadata(&self, address: &Address, networks: impl IntoIterator<Item=SupportedNetworks>) -> eyre::Result<AvatarInfoWithMetadataResponse> {
        let mut response = AvatarInfoWithMetadataResponse::default();

        let networks: Vec<SupportedNetworks> = networks.into_iter().collect();

        for network in networks {
            let provider = match network {
                SupportedNetworks::Ethereum => rpc::ethereum::new(),
                SupportedNetworks::Sepolia => rpc::sepolia::new(),
                SupportedNetworks::Polygon => rpc::polygon::new(),
                SupportedNetworks::Base => rpc::base::new(),
            };
            
            let maybe_avatar_info = provider.get_avatar_info_with_metadata(address, self.cache.clone()).await.ok();

            response.networks.insert(network.to_string().to_lowercase(), [(AvatarType::Flat, maybe_avatar_info)].into());
        }

        Ok(response)
    }

    pub async fn listen_contract_events(&self) {

    }
}
