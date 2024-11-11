use std::future::Future;

use super::models::{
    company_validator::{AttachProfessionalInCompany, CreateCompany},
    message::MessageProfessionalValidated,
    Company, CompanyError,
};

pub trait CompanyRepository: Clone + Send + Sync + 'static {
    fn find_by_id(
        &self,
        id: String,
    ) -> impl Future<Output = Result<Option<Company>, CompanyError>> + Send;
    fn find_all(&self) -> impl Future<Output = Result<Vec<Company>, CompanyError>> + Send;
    fn save(&self, company: &Company) -> impl Future<Output = Result<(), CompanyError>> + Send;
    fn delete(&self, id: uuid::Uuid) -> impl Future<Output = Result<(), CompanyError>> + Send;
    fn add_professional_to_company(
        &self,
        company_id: String,
        professional_id: String,
    ) -> impl Future<Output = Result<(), CompanyError>> + Send;
}

pub trait CompanyService: Clone + Send + Sync + 'static {
    fn create(
        &self,
        payload: CreateCompany,
    ) -> impl Future<Output = Result<Company, CompanyError>> + Send;
    fn find_by_id(
        &self,
        id: String,
    ) -> impl Future<Output = Result<Option<Company>, CompanyError>> + Send;
    fn find_all(&self) -> impl Future<Output = Result<Vec<Company>, CompanyError>> + Send;
    fn add_professional_to_company(
        &self,
        company_id: String,
        professional_data: AttachProfessionalInCompany,
    ) -> impl Future<Output = Result<(), CompanyError>> + Send;
    fn handle_professional_validated(
        &self,
        message: MessageProfessionalValidated,
    ) -> impl Future<Output = Result<(), CompanyError>> + Send;
}
