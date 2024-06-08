use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

pub struct AppError(pub eyre::Report);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

impl<E: Into<eyre::Report>> From<E> for AppError {
    fn from(error: E) -> Self {
        AppError(error.into())
    }
}

pub type AppResult<T, E = AppError> = Result<T, E>;