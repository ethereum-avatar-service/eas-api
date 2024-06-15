use std::collections::HashMap;
use std::sync::{Arc, LazyLock};

use alloy::primitives::Address;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::Deserialize;

use crate::models::avatar::AvatarCollection;
use crate::response::error::AppResult;
use crate::services::avatar::AvatarService;
use crate::supported_networks::SupportedNetworks;

static KEY: LazyLock<String> = LazyLock::new(|| {
    std::env::var("KEY").expect("KEY not set")
});

#[allow(clippy::missing_errors_doc)]
pub async fn get(State(avatar_service): State<Arc<AvatarService>>) -> AppResult<Json<HashMap<SupportedNetworks, HashMap<Address, AvatarCollection>>>> {
    let response = avatar_service.cache.verified_collections.read().await.clone();

    Ok(Json(response))
}

#[derive(Deserialize)]
pub struct ReloadParams {
    key: String
}

pub async fn reload(State(avatar_service): State<Arc<AvatarService>>, Json(params): Json<ReloadParams>) -> Response {
    if params.key == *KEY {
        avatar_service.reload_verified_collections().await;

        (StatusCode::OK, "Reloaded whitelist").into_response()
    } else {
        (StatusCode::FORBIDDEN, "Wrong key").into_response()
    }
}