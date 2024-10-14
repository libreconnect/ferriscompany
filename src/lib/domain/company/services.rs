use super::{
    models::{Company, CompanyError},
    ports::{CompanyRepository, CompanyService},
};

#[derive(Debug, Clone)]
pub struct CompanyServiceImpl<C>
where
    C: CompanyRepository,
{
    company_repository: C,
}

impl<C> CompanyServiceImpl<C>
where
    C: CompanyRepository,
{
    pub fn new(company_repository: C) -> CompanyServiceImpl<C> {
        CompanyServiceImpl { company_repository }
    }
}

impl<C> CompanyService for CompanyServiceImpl<C>
where
    C: CompanyRepository,
{
    async fn create(&self, name: &str) -> Result<Company, CompanyError> {
        let company = Company::new(name);
        self.company_repository.save(&company).await?;

        Ok(company)
    }
}
