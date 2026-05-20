//! Server startup: load config, wire the provider and cache, serve.

use std::sync::Arc;

use defi_vault_explorer_api::app;
use defi_vault_explorer_api::config::Config;
use defi_vault_explorer_api::providers::morpho::MorphoProvider;
use defi_vault_explorer_api::state::AppState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = dotenvy::dotenv();
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "defi_vault_explorer_api=info,tower_http=info".into()),
        )
        .init();

    let config = Config::from_env();
    let provider = Arc::new(MorphoProvider::new(
        config.morpho_api_url.clone(),
        config.vault_cap,
    ));
    let state = AppState::new(provider, config.cache_ttl);

    let listener = tokio::net::TcpListener::bind(&config.bind_address).await?;
    tracing::info!(bind_address = %config.bind_address, "starting API server");
    axum::serve(listener, app(state)).await?;

    Ok(())
}
