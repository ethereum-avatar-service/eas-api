use std::sync::LazyLock;

use alloy::primitives::Address;

use crate::services::rpc::Client;
use crate::supported_networks::SupportedNetworks;

static BASE_RPC_URL: LazyLock<String> = LazyLock::new(|| {
    std::env::var("BASE_RPC_URL").expect("BASE_RPC_URL not set")
});

static BASE_AVATAR_SERVICE: LazyLock<String> = LazyLock::new(|| {
    std::env::var("BASE_AVATAR_SERVICE").expect("BASE_AVATAR_SERVICE not set")
});

pub fn new() -> Client {
    let contract_address = BASE_AVATAR_SERVICE.parse::<Address>().unwrap();
    
    Client::new(SupportedNetworks::Ethereum, &BASE_RPC_URL, contract_address).unwrap()
}