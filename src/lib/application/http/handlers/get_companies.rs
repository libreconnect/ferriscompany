use std::sync::Arc;

use axum::{http::StatusCode, Extension};

use crate::domain::company::{models::Company, ports::CompanyService};

use super::{ApiError, ApiSuccess};

pub async fn get_companies<C: CompanyService>(
    Extension(company_service): Extension<Arc<C>>,
) -> Result<ApiSuccess<Vec<Company>>, ApiError> {
    company_service
        .find_all()
        .await
        .map_err(ApiError::from)
        .map(|c| ApiSuccess::new(StatusCode::OK, c))
}
