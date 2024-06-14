use serde::{Deserialize, Serialize};
use strum::{EnumIter, IntoEnumIterator};

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Hash, EnumIter, Clone)]
pub enum SupportedNetworks {
    Sepolia,
    Polygon
}

impl SupportedNetworks {
    pub fn all() -> Vec<Self> {
        SupportedNetworks::iter().collect()
    }
}