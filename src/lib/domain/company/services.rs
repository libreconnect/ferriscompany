use std::sync::Arc;

use tracing::info;

use crate::application::ports::messaging_ports::MessagingPort;

use super::{
    models::{
        company_validator::{AttachProfessionalInCompany, CreateCompany},
        Company, CompanyError,
    },
    ports::{CompanyRepository, CompanyService},
};

#[derive(Debug, Clone)]
pub struct CompanyServiceImpl<C, M>
where
    C: CompanyRepository,
    M: MessagingPort,
{
    company_repository: C,
    messaging: Arc<M>,
}

impl<C, M> CompanyServiceImpl<C, M>
where
    C: CompanyRepository,
    M: MessagingPort,
{
    pub fn new(company_repository: C, messaging: Arc<M>) -> CompanyServiceImpl<C, M> {
        CompanyServiceImpl {
            company_repository,
            messaging,
        }
    }
}

impl<C, M> CompanyService for CompanyServiceImpl<C, M>
where
    C: CompanyRepository,
    M: MessagingPort,
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

    async fn find_all(&self) -> Result<Vec<Company>, CompanyError> {
        self.company_repository.find_all().await
    }

    async fn add_professional_to_company(
        &self,
        company_id: String,
        professional_data: AttachProfessionalInCompany,
    ) -> Result<(), CompanyError> {
        info!("Adding professional to company: {:?}", professional_data);
        let message = format!(
            "{{\"professional_id\": \"{}\", \"company_id\": \"{}\"}}",
            professional_data.professional_id, &company_id
        );
        self.company_repository.find_by_id(company_id).await?;

        self.messaging
            .publish_message(String::from("company.professional.add.requested"), message)
            .await
            .map_err(|e| CompanyError::Unkown(e.to_string()))?;

        Ok(())
    }

    async fn handle_professional_validated(
        &self,
        message: super::models::message::MessageProfessionalValidated,
    ) -> Result<(), CompanyError> {
        info!("Handling professional validated: {:?}", message);

        self.company_repository
            .add_professional_to_company(message.company_id, message.professional_id)
            .await?;

        Ok(())
    }
}
