//! Health route: a static liveness probe reporting the runtime data mode.

use axum::Json;
use serde::{Deserialize, Serialize};

/// Body of the `/health` response.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HealthResponse {
    pub status: String,
    pub service: String,
    pub chain: String,
    /// `"live"` since this iteration serves real Morpho data.
    pub data_mode: String,
}

/// `GET /health` — report that the service is up and serving live data.
pub async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
        service: "defi-vault-explorer-api".to_string(),
        chain: "base".to_string(),
        data_mode: "live".to_string(),
    })
}
