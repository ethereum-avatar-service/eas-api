use std::sync::LazyLock;

use alloy::primitives::Address;

use crate::services::rpc::Client;
use crate::supported_networks::SupportedNetworks;

static ETHEREUM_RPC_URL: LazyLock<String> = LazyLock::new(|| {
    std::env::var("ETHEREUM_RPC_URL").expect("ETHEREUM_RPC_URL not set")
});

static ETHEREUM_AVATAR_SERVICE: LazyLock<String> = LazyLock::new(|| {
    std::env::var("ETHEREUM_AVATAR_SERVICE").expect("ETHEREUM_AVATAR_SERVICE not set")
});

pub fn new() -> Client {
    let contract_address = ETHEREUM_AVATAR_SERVICE.parse::<Address>().unwrap();
    
    Client::new(SupportedNetworks::Ethereum, &ETHEREUM_RPC_URL, contract_address).unwrap()
}