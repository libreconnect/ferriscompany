use std::sync::Arc;

use clap::Parser;
use ferriscompany::{
    application::http::{HttpServer, HttpServerConfig},
    domain::company::services::CompanyServiceImpl,
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

    let _nats = Arc::new(Nats::new(&env.nats_url).await?);

    let server_config = HttpServerConfig::new(env.port.clone());

    let company_repository = Neo4jCompanyRepository::new(Arc::clone(&neo4j));
    let company_service = CompanyServiceImpl::new(company_repository, Arc::clone(&messaging));

    let http_server = HttpServer::new(server_config, Arc::new(company_service)).await?;

    http_server.run().await
}
