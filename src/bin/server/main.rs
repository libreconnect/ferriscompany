use std::sync::Arc;

use ferriscompany::{
    application::http::{HttpServer, HttpServerConfig},
    domain::company::services::CompanyServiceImpl,
    infrastructure::{
        company::neo4j::company_repository::Neo4jCompanyRepository, db::neo4j::Neo4j,
    },
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    let database = Neo4j::new("0.0.0.0:7687", "neo4j", "password").await;
    let neo4j = Arc::new(database);

    let server_config = HttpServerConfig { port: "3333" };

    let company_repository = Neo4jCompanyRepository::new(Arc::clone(&neo4j));
    let company_service = CompanyServiceImpl::new(company_repository);

    let http_server = HttpServer::new(server_config, Arc::new(company_service)).await?;

    http_server.run().await
}
