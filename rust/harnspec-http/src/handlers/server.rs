//! Server management handlers

use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;

use crate::state::AppState;

/// Shutdown the server
pub async fn shutdown(State(state): State<AppState>) -> impl IntoResponse {
    tracing::info!("Shutdown request received via API");

    // Signal shutdown
    let _ = state.shutdown_tx.send(());

    (StatusCode::OK, "Server shutting down...")
}
