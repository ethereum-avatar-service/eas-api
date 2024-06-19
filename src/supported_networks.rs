use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, IntoEnumIterator};

#[derive(Deserialize, Serialize, Debug, Display, PartialEq, Eq, Hash, EnumIter, Clone)]
pub enum SupportedNetworks {
    Ethereum,
    Sepolia,
    Polygon,
    Base
}

impl SupportedNetworks {
    pub fn all() -> Vec<Self> {
        SupportedNetworks::iter().collect()
    }
}