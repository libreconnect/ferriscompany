use std::future::Future;

use super::models::{Company, CompanyError};

pub trait CompanyRepository: Clone + Send + Sync + 'static {
    fn find_by_id(
        &self,
        id: uuid::Uuid,
    ) -> impl Future<Output = Result<Option<Company>, CompanyError>> + Send;
    fn find_all(&self) -> impl Future<Output = Result<Vec<Company>, CompanyError>> + Send;
    fn save(&self, company: &Company) -> impl Future<Output = Result<(), CompanyError>> + Send;
    fn delete(&self, id: uuid::Uuid) -> impl Future<Output = Result<(), CompanyError>> + Send;
}

pub trait CompanyService: Clone + Send + Sync + 'static {
    fn create(&self, name: &str) -> impl Future<Output = Result<Company, CompanyError>> + Send;
}
