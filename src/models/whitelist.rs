use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Collection {
    pub contract: String,
    pub name: String,
    pub author: String,
    pub website: String,
    pub opensea: Option<String>
}

#[derive(Deserialize, Debug)]
pub struct Collections(pub HashMap<String, Vec<Collection>>);