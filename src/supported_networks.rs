use strum::{EnumIter, IntoEnumIterator};

#[derive(Debug, PartialEq, Eq, Hash, EnumIter)]
pub enum SupportedNetworks {
    Sepolia,
    Polygon
}

impl SupportedNetworks {
    pub fn all() -> Vec<Self> {
        SupportedNetworks::iter().collect()
    }
}