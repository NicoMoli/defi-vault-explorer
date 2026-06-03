//! Server startup: load config, wire the provider and cache, spawn the
//! background refresh task, and serve until a shutdown signal arrives.

use std::sync::Arc;

use defi_vault_explorer_api::app;
use defi_vault_explorer_api::config::Config;
use defi_vault_explorer_api::providers::morpho::MorphoProvider;
use defi_vault_explorer_api::refresh;
use defi_vault_explorer_api::state::AppState;
use tokio_util::sync::CancellationToken;

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
    let state = AppState::new(provider);

    // One shared token drives shutdown for both the refresh task and the server.
    let token = CancellationToken::new();
    let refresh_handle = tokio::spawn(refresh::run(
        state.clone(),
        config.refresh_interval,
        token.clone(),
    ));

    let listener = tokio::net::TcpListener::bind(&config.bind_address).await?;
    tracing::info!(bind_address = %config.bind_address, "starting API server");

    let shutdown_token = token.clone();
    axum::serve(listener, app(state))
        .with_graceful_shutdown(async move {
            shutdown_signal().await;
            // Stop the refresh task on the same signal that drains the server.
            shutdown_token.cancel();
        })
        .await?;

    // Let an in-flight refresh finish rather than killing it mid-fetch.
    if let Err(err) = refresh_handle.await {
        tracing::warn!(%err, "refresh task did not shut down cleanly");
    }

    Ok(())
}

/// Resolve when the process receives Ctrl-C or (on Unix) SIGTERM.
///
/// Interactive runs send SIGINT (Ctrl-C); orchestrators (Docker, Kubernetes,
/// systemd) send SIGTERM. We listen for both so graceful shutdown fires in
/// either case rather than falling through to the default instant-kill handler.
async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("install Ctrl-C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("install SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {}
        _ = terminate => {}
    }

    tracing::info!("shutdown signal received");
}
