use serde::Deserialize;

#[derive(Default, Deserialize, Clone)]
pub struct NftMetadata {
    pub image: Option<String>
}