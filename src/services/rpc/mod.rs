use std::sync::{Arc, LazyLock};

use alloy::primitives::{Address, FixedBytes, U256};
use alloy::providers::{ProviderBuilder, ReqwestProvider};
use alloy::sol;
use thiserror::Error;

use crate::models::avatar::{AvatarCollection, AvatarInfo, AvatarInfoWithMetadata, AvatarMetadata};
use crate::models::nft::NftMetadata;
use crate::services::avatar::AvatarServiceCache;
use crate::supported_networks::SupportedNetworks;

pub mod sepolia;
pub mod polygon;

static DEFAULT_AVATAR_IMAGE: LazyLock<String> = LazyLock::new(|| {
    std::env::var("DEFAULT_AVATAR_IMAGE").expect("DEFAULT_AVATAR_IMAGE not set")
});

static IPFS_GATEWAY: LazyLock<String> = LazyLock::new(|| {
    std::env::var("IPFS_GATEWAY").expect("IPFS_GATEWAY not set")
});

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
        
        let mut avatar_metadata = self.get_avatar_metadata(&avatar_info.avatar.token_address, avatar_info.avatar.token_id).await?;
        
        if let Some(network) = cache.verified_collections.read().await.get(&self.chain) {
            if let Some(collection) = network.get(&avatar_info.avatar.token_address) {
                avatar_metadata.collection = Some(collection.clone());
            }
        }

        Ok(AvatarInfoWithMetadata {
            avatar: avatar_info.avatar,
            owned: avatar_info.owned,
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
    async fn get_metadata_from_token_uri(&self, token_uri: &str) -> eyre::Result<AvatarMetadata> {
        const MAX_RETRIES: u8 = 4;

        let mut retries = 0;
        
        if token_uri.is_empty() {
            return Err(Error::EmptyTokenUri.into());
        }

        let nft_metadata = loop {
            let token_uri = token_uri.replace("ipfs://", "");
            let url = format!("{}{}", *IPFS_GATEWAY, token_uri);

            match reqwest::get(&url).await {
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

        Ok(AvatarMetadata {
            image: nft_metadata.image,
            collection: Some(AvatarCollection {
                name: None,
                author: None,
                website: None,
                opensea: None,
                verified: false,
            })
        })
    }

    #[allow(clippy::missing_errors_doc)]
    pub async fn get_avatar_metadata(&self, token_address: &Address, token_id: U256) -> eyre::Result<AvatarMetadata> {
        if *token_address == Address::ZERO {
            return Ok(AvatarMetadata {
                image: DEFAULT_AVATAR_IMAGE.to_string(),
                collection: Some(AvatarCollection {
                    name: None,
                    author: None,
                    website: None,
                    opensea: None,
                    verified: false,
                })
            })
        }
        
        let token_uri = self.get_token_uri(token_address, token_id).await?;
        self.get_metadata_from_token_uri(&token_uri).await
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

        let metadata = client.get_metadata_from_token_uri(token_uri).await.unwrap();

        assert_eq!(metadata.image, "ipfs://Qmdzin1M19QMnVUzzvNbvPKTrDezX8oPVhJj4H6nx9x7pF");
    }
}
