use std::sync::Arc;

use clap::Parser;
use ferriscompany::{
    application::{
        http::{HttpServer, HttpServerConfig},
        ports::{
            messaging_ports::MessagingPort, messaging_subscriber_port::MessagingSubscriberPort,
        },
    },
    domain::company::{
        models::message::MessageProfessionalValidated,
        ports::{CompanyRepository, CompanyService},
        services::CompanyServiceImpl,
    },
    env::Env,
    infrastructure::{
        company::neo4j::company_repository::Neo4jCompanyRepository,
        db::neo4j::Neo4j,
        messaging::nats::{Nats, NatsMessaging},
    },
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    let env = Arc::new(Env::parse());

    let database = Neo4j::new(
        &env.database_url,
        &env.database_user,
        &env.database_password,
    )
    .await;
    let neo4j = Arc::new(database);

    let messaging = Arc::new(NatsMessaging::new(&env.nats_url).await?);

    let server_config = HttpServerConfig::new(env.port.clone());

    let company_repository = Neo4jCompanyRepository::new(Arc::clone(&neo4j));
    let company_service = Arc::new(CompanyServiceImpl::new(
        company_repository,
        Arc::clone(&messaging),
    ));

    start_subscriptions(Arc::clone(&company_service), Arc::clone(&messaging)).await;

    let http_server = HttpServer::new(server_config, Arc::clone(&company_service)).await?;

    http_server.run().await
}

async fn start_subscriptions<C, M>(
    company_service: Arc<CompanyServiceImpl<C, M>>,
    messaging: Arc<NatsMessaging>,
) where
    C: CompanyRepository,
    M: MessagingPort,
{
    let company_service = Arc::clone(&company_service);
    let messaging = Arc::clone(&messaging);

    tokio::spawn(async move {
        let result = messaging
            .subscribe(
                "professional.add.validated",
                move |e: MessageProfessionalValidated| {
                    let t = Arc::clone(&company_service);

                    async move {
                        let company_service = Arc::clone(&t);
                        let _ = company_service.handle_professional_validated(e).await;
                        Ok(())
                    }
                },
            )
            .await;

        if let Err(e) = result {
            tracing::error!("Error subscribing to topic: {:?}", e);
        }
    });
}
