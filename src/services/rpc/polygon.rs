use std::sync::LazyLock;

use alloy::primitives::Address;

use crate::services::rpc::Client;

static POLYGON_RPC_URL: LazyLock<String> = LazyLock::new(|| {
    std::env::var("POLYGON_RPC_URL").expect("POLYGON_RPC_URL not set")
});

static POLYGON_AVATAR_SERVICE: LazyLock<String> = LazyLock::new(|| {
    std::env::var("POLYGON_AVATAR_SERVICE").expect("POLYGON_AVATAR_SERVICE not set")
});

pub fn new() -> Client {
    let contract_address = POLYGON_AVATAR_SERVICE.parse::<Address>().unwrap();
    
    Client::new(&POLYGON_RPC_URL, contract_address).unwrap()
}