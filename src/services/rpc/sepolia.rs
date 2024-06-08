use std::sync::LazyLock;

use alloy::primitives::Address;

use crate::services::rpc::Client;

static SEPOLIA_RPC_URL: LazyLock<String> = LazyLock::new(|| {
    std::env::var("SEPOLIA_RPC_URL").expect("SEPOLIA_RPC_URL not set")
});

static SEPOLIA_AVATAR_SERVICE: LazyLock<String> = LazyLock::new(|| {
    std::env::var("SEPOLIA_AVATAR_SERVICE").expect("SEPOLIA_AVATAR_SERVICE not set")
});

pub fn new() -> Client {
    let contract_address = SEPOLIA_AVATAR_SERVICE.parse::<Address>().unwrap();
    
    Client::new(&SEPOLIA_RPC_URL, contract_address).unwrap()
}