//! Vault routes: the cached, scored list and per-address lookup.

use axum::Json;
use axum::extract::{Path, State};

use crate::domain::Vault;
use crate::error::ApiError;
use crate::state::AppState;

/// `GET /vaults` — the whitelisted Base vault list, cached and scored.
pub async fn list_vaults(State(state): State<AppState>) -> Result<Json<Vec<Vault>>, ApiError> {
    Ok(Json(state.vaults().await?))
}

/// `GET /vaults/{address}` — one vault by address, served from the same cache.
pub async fn get_vault(
    State(state): State<AppState>,
    Path(address): Path<String>,
) -> Result<Json<Vault>, ApiError> {
    Ok(Json(state.vault_by_address(&address).await?))
}
