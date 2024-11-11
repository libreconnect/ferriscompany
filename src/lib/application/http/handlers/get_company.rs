use std::sync::Arc;

use axum::{extract::Path, http::StatusCode, Extension};

use crate::domain::company::ports::CompanyService;

use super::{ApiError, ApiSuccess};

pub async fn get_company<C: CompanyService>(
    Extension(company_service): Extension<Arc<C>>,
    Path(company_id): Path<String>,
) -> Result<ApiSuccess<String>, ApiError> {
    company_service
        .find_by_id(company_id)
        .await
        .map_err(ApiError::from)
        .map(|company| ApiSuccess::new(StatusCode::OK, company.unwrap().id.to_string()))
}
