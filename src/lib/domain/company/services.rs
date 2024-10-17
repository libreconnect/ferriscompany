use tracing::info;

use super::{
    models::{
        company_validator::{AttachProfessionalInCompany, CreateCompany},
        Company, CompanyError,
    },
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
    async fn create(&self, payload: CreateCompany) -> Result<Company, CompanyError> {
        let company = Company::new_from(payload)?;

        info!("Creating company: {:?}", company);
        self.company_repository.save(&company).await?;

        Ok(company)
    }

    async fn find_by_id(&self, id: String) -> Result<Option<Company>, CompanyError> {
        let company = self.company_repository.find_by_id(id).await?;
        Ok(company)
    }

    async fn add_professional_to_company(
        &self,
        company_id: String,
        professional_data: AttachProfessionalInCompany,
    ) -> Result<(), CompanyError> {
        info!("Adding professional to company: {:?}", professional_data);
        self.company_repository.find_by_id(company_id).await?;
        Ok(())
    }
}
