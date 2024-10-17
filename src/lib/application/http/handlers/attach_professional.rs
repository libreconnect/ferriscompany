use std::sync::Arc;

use axum::{extract::Path, http::StatusCode, Extension, Json};
use serde::Serialize;

use crate::domain::company::{
    models::company_validator::AttachProfessionalInCompany, ports::CompanyService,
};

use super::{ApiError, ApiSuccess};

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct AttachProfessionalResponseData {
    message: String,
}

pub async fn attach_professional<C: CompanyService>(
    Extension(company_service): Extension<Arc<C>>,
    Path(company_id): Path<String>,
    Json(body): Json<AttachProfessionalInCompany>,
) -> Result<ApiSuccess<AttachProfessionalResponseData>, ApiError> {
    company_service
        .add_professional_to_company(company_id, body)
        .await
        .map_err(ApiError::from)
        .map(|_| {
            ApiSuccess::new(
                StatusCode::ACCEPTED,
                AttachProfessionalResponseData {
                    message: "Request to add professional to company has been accepted".to_string(),
                },
            )
        })
}
