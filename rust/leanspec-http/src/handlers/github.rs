//! GitHub integration API handlers

use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};

use crate::state::AppState;

use leanspec_core::github::{GitHubClient, GitHubRepo, RepoRef, SpecDetectionResult};

/// POST /api/github/detect - Detect specs in a GitHub repository
pub async fn github_detect_specs(
    State(_state): State<AppState>,
    Json(body): Json<DetectRequest>,
) -> Result<Json<DetectResponse>, (StatusCode, String)> {
    let repo_ref = RepoRef::parse(&body.repo).ok_or_else(|| {
        (
            StatusCode::BAD_REQUEST,
            format!(
                "Invalid repository reference: '{}'. Use 'owner/repo' or a GitHub URL.",
                body.repo
            ),
        )
    })?;

    let client = make_client(body.token.as_deref());

    let result =
        tokio::task::spawn_blocking(move || client.detect_specs(&repo_ref, body.branch.as_deref()))
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
            .map_err(|e| (StatusCode::BAD_GATEWAY, e.to_string()))?;

    Ok(Json(DetectResponse { result }))
}

/// GET /api/github/repos - List repos accessible to the authenticated user
pub async fn github_list_repos(
    State(_state): State<AppState>,
) -> Result<Json<ListReposResponse>, (StatusCode, String)> {
    let client = make_client(None);

    let repos = tokio::task::spawn_blocking(move || client.list_user_repos(30))
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .map_err(|e| (StatusCode::BAD_GATEWAY, e.to_string()))?;

    Ok(Json(ListReposResponse { repos }))
}

/// POST /api/github/import - Import a GitHub repo as a LeanSpec project
pub async fn github_import_repo(
    State(state): State<AppState>,
    Json(body): Json<ImportRequest>,
) -> Result<Json<ImportResponse>, (StatusCode, String)> {
    let repo_ref = RepoRef::parse(&body.repo).ok_or_else(|| {
        (
            StatusCode::BAD_REQUEST,
            format!("Invalid repository reference: '{}'", body.repo),
        )
    })?;

    let client = make_client(body.token.as_deref());

    // Detect specs first if no specs_path provided
    let (branch, specs_path) =
        if let (Some(branch), Some(path)) = (body.branch.as_deref(), body.specs_path.as_deref()) {
            (branch.to_string(), path.to_string())
        } else {
            let repo_ref_clone = repo_ref.clone();
            let branch_clone = body.branch.clone();
            let detection = tokio::task::spawn_blocking(move || {
                client.detect_specs(&repo_ref_clone, branch_clone.as_deref())
            })
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
            .map_err(|e| (StatusCode::BAD_GATEWAY, e.to_string()))?;

            match detection {
                Some(result) => (result.branch, result.specs_dir),
                None => {
                    return Err((
                        StatusCode::NOT_FOUND,
                        format!("No specs found in repository '{}'", body.repo),
                    ))
                }
            }
        };

    // Register in project registry
    let mut registry = state.registry.write().await;
    let project = registry
        .add_github(
            &repo_ref.full_name(),
            &branch,
            &specs_path,
            body.name.as_deref(),
        )
        .map_err(|e| (StatusCode::CONFLICT, e.to_string()))?;

    // Sync specs from GitHub into the local cache
    let client2 = make_client(body.token.as_deref());
    let repo_ref2 = repo_ref.clone();
    let branch2 = branch.clone();
    let specs_path2 = specs_path.clone();
    let specs_dir = project.specs_dir.clone();

    let synced = tokio::task::spawn_blocking(move || {
        sync_specs_to_cache(&client2, &repo_ref2, &branch2, &specs_path2, &specs_dir)
    })
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .map_err(|e| (StatusCode::BAD_GATEWAY, e.to_string()))?;

    Ok(Json(ImportResponse {
        project_id: project.id,
        project_name: project.name,
        repo: repo_ref.full_name(),
        branch,
        specs_path,
        synced_specs: synced,
    }))
}

/// POST /api/github/sync/{id} - Sync specs from GitHub for a project
pub async fn github_sync_project(
    State(state): State<AppState>,
    axum::extract::Path(project_id): axum::extract::Path<String>,
) -> Result<Json<SyncResponse>, (StatusCode, String)> {
    let registry = state.registry.read().await;
    let project = registry
        .get(&project_id)
        .ok_or_else(|| (StatusCode::NOT_FOUND, "Project not found".to_string()))?;

    let github_config = project.github.as_ref().ok_or_else(|| {
        (
            StatusCode::BAD_REQUEST,
            "Project is not a GitHub project".to_string(),
        )
    })?;

    let client = make_client(None);
    let repo_ref = RepoRef::parse(&github_config.repo).ok_or_else(|| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Invalid stored repo reference".to_string(),
        )
    })?;

    let branch = github_config.branch.clone();
    let specs_path = github_config.specs_path.clone();
    let specs_dir = project.specs_dir.clone();

    drop(registry);

    let synced = tokio::task::spawn_blocking(move || {
        sync_specs_to_cache(&client, &repo_ref, &branch, &specs_path, &specs_dir)
    })
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .map_err(|e| (StatusCode::BAD_GATEWAY, e.to_string()))?;

    Ok(Json(SyncResponse {
        project_id,
        synced_specs: synced,
    }))
}

/// Sync specs from a GitHub repo into a local cache directory.
fn sync_specs_to_cache(
    client: &GitHubClient,
    repo_ref: &RepoRef,
    branch: &str,
    specs_path: &str,
    local_specs_dir: &std::path::Path,
) -> Result<usize, leanspec_core::CoreError> {
    let items = client.list_contents(repo_ref, specs_path, Some(branch))?;
    let mut synced = 0;

    for item in &items {
        if item.item_type != "dir" {
            continue;
        }
        // Only process numbered spec directories
        if !item.name.chars().next().is_some_and(|c| c.is_ascii_digit()) {
            continue;
        }

        let local_dir = local_specs_dir.join(&item.name);
        std::fs::create_dir_all(&local_dir)
            .map_err(|e| leanspec_core::CoreError::Other(format!("Failed to create dir: {}", e)))?;

        let readme_path = format!("{}/{}/README.md", specs_path, item.name);
        match client.get_file_content(repo_ref, &readme_path, Some(branch)) {
            Ok(content) => {
                std::fs::write(local_dir.join("README.md"), &content).map_err(|e| {
                    leanspec_core::CoreError::Other(format!("Failed to write spec: {}", e))
                })?;
                synced += 1;
            }
            Err(_) => continue,
        }
    }

    Ok(synced)
}

fn make_client(token: Option<&str>) -> GitHubClient {
    match token {
        Some(t) => GitHubClient::with_token(t),
        None => GitHubClient::new(),
    }
}

// ── Request/Response types ───────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct DetectRequest {
    pub repo: String,
    #[serde(default)]
    pub branch: Option<String>,
    #[serde(default)]
    pub token: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct DetectResponse {
    pub result: Option<SpecDetectionResult>,
}

#[derive(Debug, Serialize)]
pub struct ListReposResponse {
    pub repos: Vec<GitHubRepo>,
}

#[derive(Debug, Deserialize)]
pub struct ImportRequest {
    pub repo: String,
    #[serde(default)]
    pub branch: Option<String>,
    #[serde(default)]
    pub specs_path: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub token: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportResponse {
    pub project_id: String,
    pub project_name: String,
    pub repo: String,
    pub branch: String,
    pub specs_path: String,
    pub synced_specs: usize,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncResponse {
    pub project_id: String,
    pub synced_specs: usize,
}
