//! Route tests: `/health`, `/vaults`, `/vaults/{address}`, and the 503
//! cold-start path — all offline, against fake providers.

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

use axum::body::{Body, to_bytes};
use axum::http::{Request, StatusCode};
use defi_vault_explorer_api::app;
use defi_vault_explorer_api::domain::Vault;
use defi_vault_explorer_api::error::ApiError;
use defi_vault_explorer_api::providers::morpho::{GraphqlResponse, normalize};
use defi_vault_explorer_api::providers::{VaultFetch, VaultProvider};
use defi_vault_explorer_api::state::AppState;
use serde_json::Value;
use tower::ServiceExt;

/// A provider that returns a fixed vault list without any network call.
struct FixtureProvider {
    vaults: Vec<Vault>,
}

impl VaultProvider for FixtureProvider {
    fn fetch_vaults(&self) -> VaultFetch<'_> {
        let vaults = self.vaults.clone();
        Box::pin(async move { Ok(vaults) })
    }
}

/// A provider that always fails, to exercise the cold-start error path.
struct FailingProvider;

impl VaultProvider for FailingProvider {
    fn fetch_vaults(&self) -> VaultFetch<'_> {
        Box::pin(async { Err(ApiError::upstream_unavailable("test: upstream down")) })
    }
}

fn fixture_vaults() -> Vec<Vault> {
    let raw = include_str!("fixtures/morpho-vaults-cases.json");
    let response: GraphqlResponse = serde_json::from_str(raw).expect("fixture deserializes");
    normalize(response, "2025-06-15T00:00:00+00:00", 1_750_000_000).expect("fixture normalizes")
}

fn fixture_state() -> AppState {
    let provider = Arc::new(FixtureProvider {
        vaults: fixture_vaults(),
    });
    AppState::new(provider)
}

async fn get(state: AppState, uri: &str) -> (StatusCode, Value) {
    let response = app(state)
        .oneshot(
            Request::builder()
                .uri(uri)
                .body(Body::empty())
                .expect("request builds"),
        )
        .await
        .expect("router responds");
    let status = response.status();
    let bytes = to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("body readable");
    let json = serde_json::from_slice(&bytes).expect("body is JSON");
    (status, json)
}

#[tokio::test]
async fn health_reports_live_data_mode() {
    let (status, body) = get(fixture_state(), "/health").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["data_mode"], "live");
    assert_eq!(body["chain"], "base");
}

#[tokio::test]
async fn vaults_lists_all_cached_vaults() {
    let (status, body) = get(fixture_state(), "/vaults").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body.as_array().expect("array body").len(), 6);
}

#[tokio::test]
async fn vault_by_address_returns_the_match_case_insensitively() {
    let (status, body) = get(
        fixture_state(),
        "/vaults/0X1111111111111111111111111111111111111111",
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["name"], "Normal Vault");
}

#[tokio::test]
async fn unknown_vault_address_returns_typed_404() {
    let (status, body) = get(fixture_state(), "/vaults/0xdeadbeef").await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(body["error"], "not_found");
    assert!(body["message"].is_string());
}

#[tokio::test]
async fn cold_start_upstream_failure_returns_typed_503() {
    let state = AppState::new(Arc::new(FailingProvider));
    let (status, body) = get(state, "/vaults").await;
    assert_eq!(status, StatusCode::SERVICE_UNAVAILABLE);
    assert_eq!(body["error"], "upstream_unavailable");
}

/// A provider that counts fetches and stalls briefly, so concurrent callers
/// overlap inside the fetch window — exercising the single-flight gate.
struct CountingProvider {
    vaults: Vec<Vault>,
    calls: Arc<AtomicUsize>,
}

impl VaultProvider for CountingProvider {
    fn fetch_vaults(&self) -> VaultFetch<'_> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        let vaults = self.vaults.clone();
        Box::pin(async move {
            // Hold the gate long enough that all spawned callers pile up behind it.
            tokio::time::sleep(Duration::from_millis(50)).await;
            Ok(vaults)
        })
    }
}

#[tokio::test]
async fn concurrent_cold_misses_coalesce_into_one_fetch() {
    let calls = Arc::new(AtomicUsize::new(0));
    let state = AppState::new(Arc::new(CountingProvider {
        vaults: fixture_vaults(),
        calls: calls.clone(),
    }));

    // Fire many concurrent requests against an empty cache.
    let handles: Vec<_> = (0..16)
        .map(|_| {
            let state = state.clone();
            tokio::spawn(async move { state.vaults().await })
        })
        .collect();

    for handle in handles {
        let vaults = handle.await.expect("task joins").expect("vaults served");
        assert_eq!(vaults.len(), 6);
    }

    // Single-flight: only the first miss reached the provider.
    assert_eq!(calls.load(Ordering::SeqCst), 1);
}

#[tokio::test]
async fn refresh_warms_an_empty_cache() {
    let calls = Arc::new(AtomicUsize::new(0));
    let state = AppState::new(Arc::new(CountingProvider {
        vaults: fixture_vaults(),
        calls: calls.clone(),
    }));

    state.refresh().await.expect("refresh succeeds");

    // After a refresh, reads serve from cache without another fetch.
    let vaults = state.vaults().await.expect("vaults served");
    assert_eq!(vaults.len(), 6);
    assert_eq!(calls.load(Ordering::SeqCst), 1);
}
