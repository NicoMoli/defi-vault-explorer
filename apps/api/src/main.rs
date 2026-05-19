use axum::{Json, Router, routing::get};
use serde::{Deserialize, Serialize};
use tower_http::{cors::CorsLayer, trace::TraceLayer};

const DEFAULT_BIND_ADDRESS: &str = "127.0.0.1:4000";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct HealthResponse {
    status: String,
    service: String,
    chain: String,
    data_mode: String,
}

fn app() -> Router {
    Router::new()
        .route("/health", get(health))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
        service: "defi-vault-explorer-api".to_string(),
        chain: "base".to_string(),
        data_mode: "mock".to_string(),
    })
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = dotenvy::dotenv();
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "defi_vault_explorer_api=info,tower_http=info".into()),
        )
        .init();

    let bind_address =
        std::env::var("API_BIND_ADDRESS").unwrap_or_else(|_| DEFAULT_BIND_ADDRESS.to_string());
    let listener = tokio::net::TcpListener::bind(&bind_address).await?;

    tracing::info!(%bind_address, "starting API server");
    axum::serve(listener, app()).await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{body, body::Body, http::Request, http::StatusCode};
    use tower::ServiceExt;

    #[tokio::test]
    async fn health_should_return_service_status() {
        let response = app()
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .body(Body::empty())
                    .expect("request should build"),
            )
            .await
            .expect("health route should respond");

        assert_eq!(response.status(), StatusCode::OK);

        let body = body::to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("response body should be readable");
        let payload: HealthResponse =
            serde_json::from_slice(&body).expect("health response should be valid JSON");

        assert_eq!(
            payload,
            HealthResponse {
                status: "ok".to_string(),
                service: "defi-vault-explorer-api".to_string(),
                chain: "base".to_string(),
                data_mode: "mock".to_string(),
            }
        );
    }
}
