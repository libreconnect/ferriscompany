use company_validator::CreateCompany;
use serde::Deserialize;
use thiserror::Error;

pub mod company_validator;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
pub struct Company {
    pub id: uuid::Uuid,
    pub name: Name,
    pub city: String,
    pub country: String,
    pub email: String,
    pub phone: String,
    pub zip_code: String,
    pub address: String,
    pub national_code: String,
}

pub struct CompanyInfo {
    pub name: String,
    pub city: String,
    pub country: String,
    pub email: String,
    pub phone: String,
    pub zip_code: String,
    pub address: String,
    pub national_code: String,
}

impl Company {
    pub fn new(info: CompanyInfo) -> Company {
        let id = uuid::Uuid::new_v4();
        let name = Name::new(&info.name).unwrap();
        Company {
            id,
            name,
            city: info.city,
            country: info.country,
            email: info.email,
            phone: info.phone,
            zip_code: info.zip_code,
            address: info.address,
            national_code: info.national_code,
        }
    }

    pub fn new_from(input: CreateCompany) -> Result<Company, CompanyError> {
        let id = uuid::Uuid::new_v4();
        let name = Name::new(&input.name)
            .map_err(|_| CompanyError::CreateError("Name cannot be empty".to_string()))?;

        Ok(Company {
            id,
            name,
            city: input.city,
            country: input.country,
            email: input.email,
            phone: input.phone,
            zip_code: input.zip_code,
            address: input.address,
            national_code: input.national_code,
        })
    }
}

#[derive(Debug, Clone, Error)]
pub enum CompanyError {
    #[error("Company not found")]
    NotFound,
    #[error("Company delete error")]
    DeleteError,
    #[error("Company create error: {0}")]
    CreateError(String),
    #[error("Company unknown error: {0}")]
    Unkown(String),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
pub struct Name(String);

#[derive(Clone, Debug, Error)]
#[error("Name cannot be empty")]
pub struct NameEmptyError;

impl Name {
    pub fn new(value: &str) -> Result<Name, NameEmptyError> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            Err(NameEmptyError)
        } else {
            Ok(Name(trimmed.to_string()))
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
