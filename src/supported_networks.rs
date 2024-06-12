use serde::Serialize;
use strum::{EnumIter, IntoEnumIterator};

#[derive(Serialize, Debug, PartialEq, Eq, Hash, EnumIter, Clone)]
pub enum SupportedNetworks {
    Sepolia,
    Polygon
}

impl SupportedNetworks {
    pub fn all() -> Vec<Self> {
        SupportedNetworks::iter().collect()
    }
}