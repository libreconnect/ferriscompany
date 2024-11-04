use axum::{http::StatusCode, response::IntoResponse};

use super::{ApiError, ApiSuccess};

pub async fn live_check() -> Result<impl IntoResponse, ApiError> {
    Ok(ApiSuccess::new(StatusCode::OK, "OK"))
}
