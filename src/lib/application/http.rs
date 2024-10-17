use std::sync::Arc;

use crate::domain::company::ports::CompanyService;
use anyhow::Context;
use axum::{routing::post, Extension};
use handlers::{attach_professional::attach_professional, create_company::create_company};
use tokio::net;
use tracing::{info, info_span};

pub mod handlers;
pub mod responses;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HttpServerConfig {
    pub port: String,
}

impl HttpServerConfig {
    pub fn new(port: String) -> Self {
        Self { port }
    }
}

#[derive(Debug, Clone)]
struct AppState<C>
where
    C: CompanyService,
{
    company_service: Arc<C>,
}

pub struct HttpServer {
    router: axum::Router,
    listener: net::TcpListener,
}

impl HttpServer {
    pub async fn new<C>(
        config: HttpServerConfig,
        company_service: Arc<C>,
    ) -> anyhow::Result<Self>
    where
        C: CompanyService,
    {
        let trace_layer = tower_http::trace::TraceLayer::new_for_http().make_span_with(
            |request: &axum::extract::Request| {
                let uri: String = request.uri().to_string();
                info_span!("http_request", method = ?request.method(), uri)
            },
        );

        let state = AppState { company_service };

        let router = axum::Router::new()
            .nest("/v1", api_routes())
            .layer(trace_layer)
            .layer(Extension(Arc::clone(&state.company_service)))
            .with_state(state);

        let listener = net::TcpListener::bind(format!("0.0.0.0:{}", config.port))
            .await
            .with_context(|| format!("Failed to bind to port {}", config.port))?;

        Ok(Self { router, listener })
    }

    pub async fn run(self) -> anyhow::Result<()> {
        info!(
            "Server is running on http://{}",
            self.listener.local_addr()?
        );

        axum::serve(self.listener, self.router)
            .await
            .context("received error while running server")?;

        Ok(())
    }
}

fn api_routes<C>() -> axum::Router<AppState<C>>
where
    C: CompanyService + Send + Sync + 'static,
{
    axum::Router::new()
        .route("/companies", post(create_company::<C>))
        .route(
            "/companies/:company_id/professionals",
            post(attach_professional::<C>),
        )
}
