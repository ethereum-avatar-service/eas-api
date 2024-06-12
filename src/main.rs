use std::sync::Arc;
use axum::{
    routing::get,
    Router,
};
use dotenv::dotenv;

use eas_api::handlers;
use eas_api::services::avatar::AvatarService;

#[tokio::main]
async fn main() {
    dotenv().ok();
    
    // initialize tracing
    tracing_subscriber::fmt::init();
    
    let avatar_service = Arc::new(AvatarService::new());
    
    // Load verified collections from GitHub: https://github.com/ethereum-avatar-service/eas-api-whitelist
    avatar_service.reload_verified_collections().await;

    let app = Router::new()
        .route("/avatar/:wallet_address", get(handlers::avatar::get))
        .route("/whitelist", get(handlers::whitelist::get))
        .with_state(avatar_service);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}