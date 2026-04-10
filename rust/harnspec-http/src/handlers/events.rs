//! SSE event handlers

use axum::body::{Body, Bytes};
use axum::extract::State;
use axum::http::{header, StatusCode};
use axum::response::Response;
use serde_json::json;
use tokio::sync::broadcast;

use crate::error::{ApiError, ApiResult};
use crate::state::AppState;
use crate::watcher::{sse_keepalive_interval, sse_min_interval, SpecChangeEvent};

/// GET /api/events/specs - server-sent events for spec changes
pub async fn spec_events(State(state): State<AppState>) -> ApiResult<Response> {
    let watcher = state.file_watcher.clone();

    let permit = state
        .sse_connections
        .clone()
        .try_acquire_owned()
        .map_err(|_| {
            (
                StatusCode::TOO_MANY_REQUESTS,
                axum::Json(ApiError::new("SSE_LIMIT", "Too many SSE connections")),
            )
        })?;

    let keepalive_interval = sse_keepalive_interval();
    let min_interval = sse_min_interval();

    let stream = async_stream::stream! {
        let _permit = permit;
        let mut keepalive = tokio::time::interval(keepalive_interval);
        let mut last_sent = std::time::Instant::now() - min_interval;

        // If watcher is disabled, we just send keep-alives to keep the connection open
        // without reporting an error. This avoids constant 503s in UI/logs.
        let mut rx = watcher.map(|w| w.subscribe());

        loop {
            tokio::select! {
                _ = keepalive.tick() => {
                    yield Ok::<Bytes, std::convert::Infallible>(Bytes::from(": keep-alive\n\n"));
                }
                result = async {
                    if let Some(ref mut rx) = rx {
                        rx.recv().await
                    } else {
                        // Sleep forever if no watcher
                        std::future::pending().await
                    }
                } => {
                    match result {
                        Ok(event) => {
                            let elapsed = last_sent.elapsed();
                            if elapsed < min_interval {
                                tokio::time::sleep(min_interval - elapsed).await;
                            }

                            last_sent = std::time::Instant::now();
                            let payload = to_sse_payload(&event);
                            yield Ok::<Bytes, std::convert::Infallible>(Bytes::from(payload));
                        }
                        Err(broadcast::error::RecvError::Lagged(_)) => {
                            continue;
                        }
                        Err(broadcast::error::RecvError::Closed) => break,
                    }
                }
            }
        }
    };

    let response = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "text/event-stream; charset=utf-8")
        .header(header::CACHE_CONTROL, "no-cache, no-transform")
        .header(header::CONNECTION, "keep-alive")
        .body(Body::from_stream(stream))
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                axum::Json(ApiError::internal_error(&e.to_string())),
            )
        })?;

    Ok(response)
}

fn to_sse_payload(event: &SpecChangeEvent) -> String {
    let data = serde_json::to_string(event).unwrap_or_else(|_| json!({}).to_string());
    format!("data: {}\n\n", data)
}
