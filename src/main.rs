use axum::{
    routing::get,
    Router,
};
use dotenv::dotenv;

use eas_api::handlers;

#[tokio::main]
async fn main() {
    dotenv().ok();
    
    // initialize tracing
    tracing_subscriber::fmt::init();

    let app = Router::new().route("/avatar/:wallet_address", get(handlers::avatar::get));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}