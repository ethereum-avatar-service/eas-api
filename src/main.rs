use std::sync::{Arc, LazyLock};

use axum::{
    routing::get,
    Router,
};
use dotenv::dotenv;
use log::info;
use tower_http::cors::{Any, CorsLayer};

use eas_api::handlers;
use eas_api::services::avatar::AvatarService;

static BIND_ADDRESS: LazyLock<String> = LazyLock::new(|| {
    std::env::var("BIND_ADDRESS").expect("BIND_ADDRESS not set")
});

#[tokio::main]
async fn main() {
    dotenv().ok();
    
    // initialize tracing
    tracing_subscriber::fmt::init();

    let avatar_service = Arc::new(AvatarService::new());

    // Load verified collections from GitHub: https://github.com/ethereum-avatar-service/eas-api-whitelist
    avatar_service.reload_verified_collections().await;

    let cors = CorsLayer::new().allow_origin(Any);

    let app = Router::new()
        .route("/avatar/:wallet_address", get(handlers::avatar::get))
        .route("/whitelist", get(handlers::whitelist::get))
        .with_state(avatar_service)
        .layer(cors);

    let listener = tokio::net::TcpListener::bind(&*BIND_ADDRESS).await.unwrap();

    info!(target: "API", "Started on: {}", *BIND_ADDRESS);
    
    axum::serve(listener, app).await.unwrap();
}