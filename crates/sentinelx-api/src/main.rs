use axum::Router;
use sentinelx_api::{routes, AppState};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("sentinelx=info".parse()?))
        .init();

    let database = std::env::var("SENTINELX_DATABASE").unwrap_or_else(|_| "sqlite:data/sentinelx.db".into());
    let state = AppState::new(&database).await?;

    let app = Router::new()
        .nest("/api/v1", routes::router())
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::info!("SentinelX API listening on http://{addr}");
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
