use std::sync::Arc;
use std::time::Duration;

use alloy::primitives::{Address, FixedBytes, U256};
use alloy::providers::{ProviderBuilder, ReqwestProvider};
use alloy::sol;
use thiserror::Error;

use crate::models::avatar::{AvatarInfo, AvatarInfoWithMetadata, AvatarMetadata};
use crate::models::nft::NftMetadata;
use crate::services::avatar::AvatarServiceCache;
use crate::supported_networks::SupportedNetworks;

pub mod sepolia;
pub mod polygon;
pub mod ethereum;

const IPFS_GATEWAYS: [&str; 4] = [
    "https://ipfs.io/ipfs/",
    "https://reddit.infura-ipfs.io/ipfs/",
    "https://cf-ipfs.com/ipfs/",
    "https://gateway.pinata.cloud/ipfs/",
];

sol!(
    #[allow(missing_docs)]
    #[allow(clippy::pub_underscore_fields)]
    #[sol(rpc)]
    AvatarService,
    "abi/AvatarService.json"
);

pub struct Client {
    chain: SupportedNetworks,
    provider: ReqwestProvider,
    avatar_service: Address
}

impl Client {
    #[allow(clippy::missing_errors_doc)]
    pub fn new(chain: SupportedNetworks, rpc_url: &str, avatar_service: Address) -> eyre::Result<Self> {
        let provider = ProviderBuilder::new().on_http(rpc_url.parse()?);

        Ok(Self { chain, provider, avatar_service })
    }

    #[allow(clippy::missing_errors_doc)]
    pub async fn get_avatar_info(&self, address: &Address) -> eyre::Result<AvatarInfo> {
        let contract = AvatarService::new(self.avatar_service, &self.provider);

        let avatar_info = contract.getAvatarInfo(*address).call().await?;

        Ok(AvatarInfo::from(avatar_info._0))
    }

    #[allow(clippy::missing_errors_doc)]
    pub async fn get_avatar_info_with_metadata(&self, address: &Address, cache: Arc<AvatarServiceCache>) -> eyre::Result<AvatarInfoWithMetadata> {
        let avatar_info = self.get_avatar_info(address).await?;

        let nft_metadata = {
            if avatar_info.avatar.token_address == Address::ZERO {
                NftMetadata::default()
            } else {
                let maybe_cached_token_uri = cache.token_uris.read().await
                    .get(&self.chain)
                    .and_then(|map| {
                        map.get(&(avatar_info.avatar.token_address, avatar_info.avatar.token_id)).cloned()
                    });

                // Try uri cache first
                let token_uri = if let Some(uri) = maybe_cached_token_uri {
                    uri
                } else {
                    let uri = self.get_token_uri(&avatar_info.avatar.token_address, avatar_info.avatar.token_id).await?;

                    // Update uri cache
                    cache.token_uris.write().await
                        .entry(self.chain.clone())
                        .or_default()
                        .insert((avatar_info.avatar.token_address, avatar_info.avatar.token_id), uri.clone());

                    uri
                };

                let opt_cached_metadata = cache.ipfs.read().await.get(&token_uri).cloned();
                
                // Try cache first
                if let Some(metadata) = opt_cached_metadata { 
                    metadata
                } else if let Ok(metadata) = self.get_nft_metadata_from_token_uri(&token_uri).await {
                    // Cache ipfs result
                    cache.ipfs.write().await.insert(token_uri, metadata.clone());
                    metadata
                } else {
                    NftMetadata::default()
                }
            }
        };
        
        let mut avatar_metadata = AvatarMetadata {
            image: nft_metadata.image,
            ..Default::default()
        };
        
        if let Some(network) = cache.verified_collections.read().await.get(&self.chain) {
            if let Some(collection) = network.get(&avatar_info.avatar.token_address) {
                avatar_metadata.collection = Some(collection.clone());
            }
        }

        Ok(AvatarInfoWithMetadata {
            avatar: avatar_info.avatar,
            owned: avatar_info.owned,
            uri: avatar_info.uri,
            avatar_metadata,
        })
    }
}

sol!(
    #[allow(clippy::pub_underscore_fields)]
    #[sol(rpc)]
    ERC165,
    r#"[{
        "constant": true,
        "inputs": [{"name": "interfaceId", "type": "bytes4"}],
        "name": "supportsInterface",
        "outputs": [{"name": "", "type": "bool"}],
        "payable": false,
        "stateMutability": "view",
        "type": "function"
    }]"#
);

sol!(
    #[allow(clippy::pub_underscore_fields)]
    #[sol(rpc)]
    ERC721,
    r#"[{
        "constant": true,
        "inputs": [{"name": "tokenId", "type": "uint256"}],
        "name": "tokenURI",
        "outputs": [{"name": "", "type": "string"}],
        "payable": false,
        "stateMutability": "view",
        "type": "function"
    }]"#
);

sol!(
    #[allow(clippy::pub_underscore_fields)]
    #[sol(rpc)]
    ERC1155,
    r#"[{
        "constant": true,
        "inputs": [{"name": "_id", "type": "uint256"}],
        "name": "uri",
        "outputs": [{"name": "", "type": "string"}],
        "payable": false,
        "stateMutability": "view",
        "type": "function"
    }]"#
);

#[derive(Error, Debug)]
pub enum Error {
    #[error("Missing token URI")]
    MissingTokenUri,
    #[error("Empty token URI")]
    EmptyTokenUri
}

impl Client {
    #[allow(clippy::missing_errors_doc)]
    async fn get_token_uri(&self, token_address: &Address, token_id: U256) -> eyre::Result<String> {
        const ERC721_INTERFACE_ID: FixedBytes<4> = FixedBytes::new([0x80, 0xac, 0x58, 0xcd]);
        const ERC1155_INTERFACE_ID: FixedBytes<4> = FixedBytes::new([0xd9, 0xb6, 0x7a, 0x26]);

        let erc165 = ERC165::new(*token_address, &self.provider);
        let is_erc721 = erc165.supportsInterface(ERC721_INTERFACE_ID).call().await.is_ok_and(|v| v._0);
        let is_erc1155 = erc165.supportsInterface(ERC1155_INTERFACE_ID).call().await.is_ok_and(|v| v._0);

        if is_erc721 {
            let erc721 = ERC721::new(*token_address, &self.provider);
            let token_uri = erc721.tokenURI(token_id).call().await?._0;
            Ok(token_uri)
        } else if is_erc1155 {
            let erc1155 = ERC1155::new(*token_address, &self.provider);
            let token_uri = erc1155.uri(token_id).call().await?._0;
            Ok(token_uri)
        } else {
            Err(Error::MissingTokenUri.into())
        }
    }

    #[allow(clippy::missing_errors_doc)]
    async fn get_nft_metadata_from_token_uri(&self, token_uri: &str) -> eyre::Result<NftMetadata> {
        const MAX_RETRIES: usize = 3;

        let mut retries = 0;
        
        if token_uri.is_empty() {
            return Err(Error::EmptyTokenUri.into());
        }

        let reqwest_client = reqwest::Client::builder()
            .timeout(Duration::from_secs(2))
            .build()
            .unwrap();

        let nft_metadata = loop {
            let gateway = IPFS_GATEWAYS[retries % IPFS_GATEWAYS.len()];
            let token_uri = token_uri.replace("ipfs://", "");
            let url = format!("{gateway}{token_uri}");

            match reqwest_client.get(&url).send().await {
                Ok(response) => match response.json::<NftMetadata>().await {
                    Ok(metadata) => break Ok(metadata),
                    Err(error) => {
                        retries += 1;
                        if retries >= MAX_RETRIES {
                            break Err(error);
                        }
                    }
                },
                Err(error) => {
                    retries += 1;
                    if retries >= MAX_RETRIES {
                        break Err(error);
                    }
                }
            }
        }?;

        Ok(nft_metadata)
    }
}

#[cfg(test)]
mod tests {
    use alloy::primitives::{address, U256};
    use dotenv::dotenv;

    use crate::services::rpc::polygon;

    #[tokio::test]
    async fn test_get_token_uri() {
        dotenv().ok();

        let client = polygon::new();

        let token_address = address!("907808732079863886443057C65827a0F1c64357");
        let token_id = U256::from(1);

        let token_uri = client.get_token_uri(&token_address, token_id).await.unwrap();

        assert!(!token_uri.is_empty());
    }

    #[tokio::test]
    async fn test_get_metadata_from_token_uri() {
        dotenv().ok();

        let client = polygon::new();

        let token_uri = "ipfs://QmNfoE5tQaBGiXSNdyRDresLC27QCHNwP75zwuXfntdBmM/1.json";

        let metadata = client.get_nft_metadata_from_token_uri(token_uri).await.unwrap();

        assert_eq!(metadata.image, Some("ipfs://Qmdzin1M19QMnVUzzvNbvPKTrDezX8oPVhJj4H6nx9x7pF".to_string()));
    }
}
