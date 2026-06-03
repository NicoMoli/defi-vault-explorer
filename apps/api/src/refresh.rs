//! Background refresh task (ADR 005).
//!
//! A single spawned task keeps the cache warm so that, in normal operation, the
//! cache is never empty and the on-demand fill in [`AppState::vaults`] never
//! fires. It refreshes through the shared single-flight gate, so it can never
//! double-fetch against an on-demand request.
//!
//! Failure handling: keep the last good snapshot, log a warning, and back off
//! exponentially (base = `refresh_interval`, ×2 per failure, capped), resetting
//! to base on the first success. No jitter — there is a single instance.
//!
//! [`AppState::vaults`]: crate::state::AppState::vaults

use std::time::Duration;

use tokio_util::sync::CancellationToken;

use crate::state::AppState;

/// Upper bound on the failure backoff delay (~10 min, per ADR 005).
const MAX_BACKOFF: Duration = Duration::from_secs(600);

/// Compute the delay until the next refresh from the previous delay and the
/// outcome of the last attempt.
///
/// Success resets to `base`; failure doubles the previous delay, capped at
/// [`MAX_BACKOFF`].
fn next_delay(previous: Duration, base: Duration, succeeded: bool) -> Duration {
    if succeeded {
        base
    } else {
        (previous * 2).min(MAX_BACKOFF)
    }
}

/// Run the refresh loop until `token` is cancelled.
///
/// The first refresh happens immediately on spawn (warming the cache without
/// blocking startup), then the loop sleeps for the computed delay, racing the
/// cancellation signal so shutdown is prompt.
pub async fn run(state: AppState, refresh_interval: Duration, token: CancellationToken) {
    let mut delay = refresh_interval;
    loop {
        let succeeded = match state.refresh().await {
            Ok(()) => {
                tracing::debug!("vault cache refreshed");
                true
            }
            Err(err) => {
                tracing::warn!(%err, "vault refresh failed; keeping last snapshot");
                false
            }
        };
        delay = next_delay(delay, refresh_interval, succeeded);

        tokio::select! {
            _ = token.cancelled() => break,
            _ = tokio::time::sleep(delay) => {}
        }
    }
    tracing::info!("vault refresh task stopped");
}

#[cfg(test)]
mod tests {
    use super::*;

    const BASE: Duration = Duration::from_secs(60);

    #[test]
    fn success_resets_to_base() {
        assert_eq!(next_delay(Duration::from_secs(240), BASE, true), BASE);
    }

    #[test]
    fn failure_doubles_the_previous_delay() {
        assert_eq!(next_delay(BASE, BASE, false), Duration::from_secs(120));
        assert_eq!(
            next_delay(Duration::from_secs(120), BASE, false),
            Duration::from_secs(240)
        );
    }

    #[test]
    fn backoff_is_capped() {
        assert_eq!(
            next_delay(Duration::from_secs(600), BASE, false),
            MAX_BACKOFF
        );
        assert_eq!(
            next_delay(Duration::from_secs(3600), BASE, false),
            MAX_BACKOFF
        );
    }
}
