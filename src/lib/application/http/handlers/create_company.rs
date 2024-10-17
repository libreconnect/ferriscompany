use std::sync::Arc;

use axum::{http::StatusCode, Extension, Json};
use serde::Serialize;
//use thiserror::Error;

use crate::domain::company::{
    models::{company_validator::CreateCompany, Company, CompanyError},
    ports::CompanyService,
};

use super::{ApiError, ApiSuccess};

// #[derive(Debug, Error)]
// enum ParseCreateCompayError {
//     #[error("missing field: {0}")]
//     MissingField(String),
//     #[error("invalid field: {0}")]
//     InvalidField(String),
// }

// impl From<ParseCreateCompayError> for ApiError {
//     fn from(e: ParseCreateCompayError) -> Self {
//       match e {
//           ParseCreateCompayError::MissingField(e) => Self::UnProcessableEntity(e.to_string()),
//           ParseCreateCompayError::InvalidField(e) => Self::UnProcessableEntity(e.to_string()),
//       }
//     }
// }

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct CreateCompanyResponseData {
    id: String,
}

impl From<&Company> for CreateCompanyResponseData {
    fn from(value: &Company) -> Self {
        CreateCompanyResponseData {
            id: value.id.to_string(),
        }
    }
}

impl From<CompanyError> for ApiError {
    fn from(e: CompanyError) -> Self {
        match e {
            CompanyError::CreateError(e) => Self::InternalServerError(e.to_string()),
            CompanyError::NotFound => Self::InternalServerError("company not found".to_string()),
            CompanyError::DeleteError => {
                Self::InternalServerError("company delete error".to_string())
            },
            CompanyError::Unkown(e) => Self::InternalServerError(e.to_string()),
        }
    }
}

pub async fn create_company<C: CompanyService>(
    Extension(company_service): Extension<Arc<C>>,
    Json(body): Json<CreateCompany>,
) -> Result<ApiSuccess<CreateCompanyResponseData>, ApiError> {
    company_service
        .create(body)
        .await
        .map_err(ApiError::from)
        .map(|ref company| ApiSuccess::new(StatusCode::CREATED, company.into()))
}
