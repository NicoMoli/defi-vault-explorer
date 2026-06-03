//! `defi-vault-explorer` API library.
//!
//! Live Morpho vault data for Base: a GraphQL provider, an in-memory TTL
//! cache, signal-by-signal risk scoring, and the HTTP routes that serve them.

pub mod cache;
pub mod config;
pub mod domain;
pub mod error;
pub mod providers;
pub mod refresh;
pub mod routes;
pub mod scoring;
pub mod state;

use axum::{Router, routing::get};
use tower_http::{cors::CorsLayer, trace::TraceLayer};

use state::AppState;

/// Build the application router with all routes and middleware wired.
pub fn app(state: AppState) -> Router {
    Router::new()
        .route("/health", get(routes::health::health))
        .route("/vaults", get(routes::vaults::list_vaults))
        .route("/vaults/{address}", get(routes::vaults::get_vault))
        .with_state(state)
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
}
