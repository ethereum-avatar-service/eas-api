use alloy::primitives::Address;
use axum::extract::{FromRequestParts, Path};
use axum::http::request::Parts;
use axum::http::StatusCode;

pub struct EthereumAddress(pub Address);

#[async_trait::async_trait]
impl<S> FromRequestParts<S> for EthereumAddress
    where
        S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let Path(address) = Path::<String>::from_request_parts(parts, state).await.map_err(|_| {
            (StatusCode::BAD_REQUEST, "Invalid path parameter")
        })?;

        match address.parse::<Address>() {
            Ok(parsed_address) => Ok(EthereumAddress(parsed_address)),
            Err(_) => Err((StatusCode::BAD_REQUEST, "Invalid Ethereum address format")),
        }
    }
}