use super::models::{Company, CompanyError};

pub trait CompanyRepository: Clone + Send + Sync + 'static {
  fn find_by_id(&self, id: uuid::Uuid) -> Result<Option<Company>, CompanyError>;
  fn find_all(&self) -> Result<Vec<Company>, CompanyError>;
  fn save(&self, company: Company) -> Result<(), CompanyError>;
  fn delete(&self, id: uuid::Uuid) -> Result<(), CompanyError>;
}