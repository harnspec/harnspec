//! Spec read handlers

#![allow(clippy::result_large_err)]

use axum::extract::{Path, Query, State};
use axum::http::{HeaderMap, StatusCode};
use axum::Json;
use std::collections::HashMap;
use std::fs;

use harnspec_core::{
    search_specs_with_options, DependencyGraph, SearchOptions, SpecFilterOptions,
    SpecHierarchyNode, SpecStatus,
};

use crate::error::{ApiError, ApiResult};
use crate::state::AppState;
use crate::sync_state::machine_id_from_headers;

use crate::types::{
    ListSpecsQuery, ListSpecsResponse, SearchRequest, SearchResponse, SpecDetail, SpecRawResponse,
    SpecSummary,
};

use super::helpers::{
    detail_from_record, detect_sub_specs, get_spec_loader, hash_raw_content, summary_from_record,
};

fn parse_status_filter(
    status: &Option<String>,
) -> Result<Option<Vec<SpecStatus>>, (StatusCode, Json<ApiError>)> {
    let parsed = status.as_ref().map(|s| {
        s.split(',')
            .filter_map(|s| s.parse().ok())
            .collect::<Vec<_>>()
    });

    if status.is_some() && parsed.as_ref().map_or(true, |v| v.is_empty()) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ApiError::invalid_request("Invalid status filter")),
        ));
    }

    Ok(parsed)
}

fn apply_pagination<T>(items: Vec<T>, offset: Option<usize>, limit: Option<usize>) -> Vec<T> {
    let start = offset.unwrap_or(0);
    let iter = items.into_iter().skip(start);
    match limit {
        Some(limit) => iter.take(limit).collect(),
        None => iter.collect(),
    }
}

fn resolve_pagination(
    query: &ListSpecsQuery,
) -> Result<(usize, Option<usize>), (StatusCode, Json<ApiError>)> {
    if let Some(cursor) = query.cursor.as_ref() {
        let offset = cursor.parse::<usize>().map_err(|_| {
            (
                StatusCode::BAD_REQUEST,
                Json(ApiError::invalid_request("Invalid cursor value")),
            )
        })?;
        let limit = Some(query.limit.unwrap_or(50));
        Ok((offset, limit))
    } else {
        Ok((query.offset.unwrap_or(0), query.limit))
    }
}

fn next_cursor(total: usize, offset: usize, limit: Option<usize>) -> Option<String> {
    let limit = limit?;
    let end = offset.saturating_add(limit);
    if end < total {
        Some(end.to_string())
    } else {
        None
    }
}

fn build_hierarchy(specs: Vec<crate::types::SpecSummary>) -> Vec<crate::types::HierarchyNode> {
    use std::collections::HashMap;

    // Create a map of spec_name -> SpecSummary
    let spec_map: HashMap<String, crate::types::SpecSummary> = specs
        .iter()
        .map(|s| (s.spec_name.clone(), s.clone()))
        .collect();

    // Create a map of parent -> children specs
    let mut children_specs: HashMap<String, Vec<crate::types::SpecSummary>> = HashMap::new();
    let mut roots: Vec<crate::types::SpecSummary> = Vec::new();

    for spec in specs {
        if let Some(parent) = &spec.parent {
            if spec_map.contains_key(parent) {
                children_specs.entry(parent.clone()).or_default().push(spec);
            } else {
                // Parent not in list, treat as root
                roots.push(spec);
            }
        } else {
            roots.push(spec);
        }
    }

    // Recursive function to build nodes
    fn build_node(
        spec: crate::types::SpecSummary,
        children_map: &HashMap<String, Vec<crate::types::SpecSummary>>,
    ) -> crate::types::HierarchyNode {
        let mut child_nodes: Vec<crate::types::HierarchyNode> = children_map
            .get(&spec.spec_name)
            .map(|children| {
                children
                    .iter()
                    .cloned()
                    .map(|c| build_node(c, children_map))
                    .collect()
            })
            .unwrap_or_default();

        // Sort children by spec_number descending (newest first)
        child_nodes.sort_by(|a, b| match (b.spec.spec_number, a.spec.spec_number) {
            (Some(bn), Some(an)) => bn.cmp(&an),
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            (None, None) => b.spec.spec_name.cmp(&a.spec.spec_name),
        });

        crate::types::HierarchyNode { spec, child_nodes }
    }

    // Build root nodes
    let mut root_nodes: Vec<crate::types::HierarchyNode> = roots
        .into_iter()
        .map(|s| build_node(s, &children_specs))
        .collect();

    // Sort roots by spec_number descending
    root_nodes.sort_by(|a, b| match (b.spec.spec_number, a.spec.spec_number) {
        (Some(bn), Some(an)) => bn.cmp(&an),
        (Some(_), None) => std::cmp::Ordering::Less,
        (None, Some(_)) => std::cmp::Ordering::Greater,
        (None, None) => b.spec.spec_name.cmp(&a.spec.spec_name),
    });

    root_nodes
}

fn sort_hierarchy_nodes(nodes: &mut [crate::types::HierarchyNode]) {
    nodes.sort_by(|a, b| match (b.spec.spec_number, a.spec.spec_number) {
        (Some(bn), Some(an)) => bn.cmp(&an),
        (Some(_), None) => std::cmp::Ordering::Less,
        (None, Some(_)) => std::cmp::Ordering::Greater,
        (None, None) => b.spec.spec_name.cmp(&a.spec.spec_name),
    });

    for node in nodes.iter_mut() {
        sort_hierarchy_nodes(&mut node.child_nodes);
    }
}

fn build_hierarchy_from_cached_tree(
    tree: &[SpecHierarchyNode],
    filtered_specs: &[crate::types::SpecSummary],
) -> Vec<crate::types::HierarchyNode> {
    use std::collections::{HashMap, HashSet};

    let allowed: HashSet<String> = filtered_specs.iter().map(|s| s.spec_name.clone()).collect();
    let spec_map: HashMap<String, crate::types::SpecSummary> = filtered_specs
        .iter()
        .map(|s| (s.spec_name.clone(), s.clone()))
        .collect();

    fn filter_nodes(
        nodes: &[SpecHierarchyNode],
        allowed: &HashSet<String>,
        spec_map: &HashMap<String, crate::types::SpecSummary>,
    ) -> Vec<crate::types::HierarchyNode> {
        let mut out = Vec::new();

        for node in nodes {
            let child_nodes = filter_nodes(&node.child_nodes, allowed, spec_map);
            if allowed.contains(&node.path) {
                if let Some(spec) = spec_map.get(&node.path) {
                    out.push(crate::types::HierarchyNode {
                        spec: spec.clone(),
                        child_nodes,
                    });
                }
            } else {
                // Promote matching descendants when the current node is filtered out.
                out.extend(child_nodes);
            }
        }

        out
    }

    let mut hierarchy = filter_nodes(tree, &allowed, &spec_map);
    sort_hierarchy_nodes(&mut hierarchy);
    hierarchy
}

/// GET /api/projects/:projectId/specs - List specs for a project
pub async fn list_project_specs(
    State(state): State<AppState>,
    Path(project_id): Path<String>,
    Query(query): Query<ListSpecsQuery>,
    headers: HeaderMap,
) -> ApiResult<Json<ListSpecsResponse>> {
    let (page_offset, page_limit) = resolve_pagination(&query)?;

    if let Some(machine_id) = machine_id_from_headers(&headers) {
        let sync_state = state.sync_state.read().await;
        let machine = sync_state
            .persistent
            .machines
            .get(&machine_id)
            .ok_or_else(|| {
                (
                    StatusCode::NOT_FOUND,
                    Json(ApiError::invalid_request("Machine not found")),
                )
            })?;

        if machine.revoked {
            return Err((
                StatusCode::FORBIDDEN,
                Json(ApiError::unauthorized("Machine revoked")),
            ));
        }

        let project = machine.projects.get(&project_id).ok_or_else(|| {
            (
                StatusCode::NOT_FOUND,
                Json(ApiError::project_not_found(&project_id)),
            )
        })?;

        let status_filter = parse_status_filter(&query.status)?.map(|values| {
            values
                .into_iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
        });

        let priority_filter = query
            .priority
            .map(|s| s.split(',').map(|v| v.to_string()).collect::<Vec<_>>());

        let tags_filter = query
            .tags
            .map(|s| s.split(',').map(|v| v.to_string()).collect::<Vec<_>>());

        // Build required_by map (which specs depend on each spec)
        // This is a reverse index: for each dependency, track which specs depend on it
        let mut required_by_map: HashMap<String, Vec<String>> = HashMap::new();
        for spec in project.specs.values() {
            for dep in &spec.depends_on {
                // Filter out self-references: a spec shouldn't be in its own required_by list
                if dep != &spec.spec_name {
                    // Log for debugging phantom dependencies
                    #[cfg(debug_assertions)]
                    eprintln!("DEBUG: Spec '{}' depends on '{}'", spec.spec_name, dep);

                    required_by_map
                        .entry(dep.clone())
                        .or_default()
                        .push(spec.spec_name.clone());
                }
            }
        }

        // Build children map (parent -> children)
        let mut children_map: HashMap<String, Vec<String>> = HashMap::new();
        for spec in project.specs.values() {
            if let Some(parent) = &spec.parent {
                children_map
                    .entry(parent.clone())
                    .or_default()
                    .push(spec.spec_name.clone());
            }
        }

        // Validate the required_by map for debugging
        #[cfg(debug_assertions)]
        for (dep, dependents) in &required_by_map {
            for dependent in dependents {
                // Sanity check: verify the dependent actually has this dep in its depends_on
                if let Some(spec) = project.specs.get(dependent) {
                    if !spec.depends_on.contains(dep) {
                        eprintln!(
                            "WARNING: Phantom dependency detected! Spec '{}' shows as depending on '{}' but doesn't have it in depends_on: {:?}",
                            dependent, dep, spec.depends_on
                        );
                    }
                }
            }
        }

        let mut filtered_specs: Vec<SpecSummary> = project
            .specs
            .values()
            .filter(|spec| {
                if let Some(statuses) = &status_filter {
                    if !statuses.contains(&spec.status) {
                        return false;
                    }
                }

                if let Some(priorities) = &priority_filter {
                    match &spec.priority {
                        Some(priority) if priorities.contains(priority) => {}
                        None if priorities.is_empty() => {}
                        _ => return false,
                    }
                }

                if let Some(tags) = &tags_filter {
                    if !tags.iter().all(|tag| spec.tags.contains(tag)) {
                        return false;
                    }
                }

                true
            })
            .map(|spec| {
                let mut summary = summary_from_record(&project.id, spec);
                let required_by = required_by_map
                    .get(&spec.spec_name)
                    .cloned()
                    .unwrap_or_default();
                summary.required_by = required_by.clone();
                summary.children = children_map
                    .get(&spec.spec_name)
                    .cloned()
                    .unwrap_or_default();
                summary.relationships = Some(crate::types::SpecRelationships {
                    depends_on: summary.depends_on.clone(),
                    required_by: Some(required_by),
                });
                summary
            })
            .collect();

        // Keep output ordering stable for pagination.
        filtered_specs.sort_by(|a, b| {
            b.spec_number
                .cmp(&a.spec_number)
                .then_with(|| b.spec_name.cmp(&a.spec_name))
        });

        let total = filtered_specs.len();
        let hierarchy_requested = query.hierarchy.unwrap_or(false);
        let paged_specs = if hierarchy_requested {
            filtered_specs.clone()
        } else {
            apply_pagination(filtered_specs.clone(), Some(page_offset), page_limit)
        };
        let next_cursor = if hierarchy_requested {
            None
        } else {
            next_cursor(total, page_offset, page_limit)
        };

        // Build hierarchy if requested - computed server-side for performance
        let hierarchy = if hierarchy_requested {
            Some(build_hierarchy(filtered_specs.clone()))
        } else {
            None
        };

        return Ok(Json(ListSpecsResponse {
            specs: paged_specs,
            total,
            next_cursor,
            project_id: Some(project.id.clone()),
            hierarchy,
        }));
    }

    let (loader, project) = get_spec_loader(&state, &project_id).await?;

    let all_specs = loader.load_all_metadata().map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiError::internal_error(&e.to_string())),
        )
    })?;
    let relationship_index = loader.load_relationship_index().map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiError::internal_error(&e.to_string())),
        )
    })?;
    let cached_tree = loader.load_hierarchy_tree().map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiError::internal_error(&e.to_string())),
        )
    })?;

    let status_filter = parse_status_filter(&query.status)?;

    // Apply filters
    let filters = SpecFilterOptions {
        status: status_filter,
        priority: query
            .priority
            .map(|s| s.split(',').filter_map(|s| s.parse().ok()).collect()),
        tags: query
            .tags
            .map(|s| s.split(',').map(|s| s.to_string()).collect()),
        assignee: query.assignee,
        search: None,
    };

    let mut filtered_specs: Vec<SpecSummary> = all_specs
        .iter()
        .filter(|s| filters.matches(s))
        .map(|s| SpecSummary::from_without_computed(s).with_project_id(&project.id))
        .collect();

    filtered_specs.sort_by(|a, b| {
        b.spec_number
            .cmp(&a.spec_number)
            .then_with(|| b.spec_name.cmp(&a.spec_name))
    });

    let filtered_specs: Vec<SpecSummary> = filtered_specs
        .into_iter()
        .map(|mut summary| {
            let required_by = relationship_index
                .required_by
                .get(&summary.spec_name)
                .cloned()
                .unwrap_or_default();
            summary.required_by = required_by.clone();
            summary.children = relationship_index
                .children_by_parent
                .get(&summary.spec_name)
                .cloned()
                .unwrap_or_default();
            summary.relationships = Some(crate::types::SpecRelationships {
                depends_on: summary.depends_on.clone(),
                required_by: Some(required_by),
            });
            summary
        })
        .collect();

    let total = filtered_specs.len();
    let hierarchy_requested = query.hierarchy.unwrap_or(false);
    let paged_specs = if hierarchy_requested {
        filtered_specs.clone()
    } else {
        apply_pagination(filtered_specs.clone(), Some(page_offset), page_limit)
    };
    let next_cursor = if hierarchy_requested {
        None
    } else {
        next_cursor(total, page_offset, page_limit)
    };

    // Build hierarchy if requested - computed server-side for performance
    let hierarchy = if hierarchy_requested {
        Some(build_hierarchy_from_cached_tree(
            &cached_tree,
            &filtered_specs,
        ))
    } else {
        None
    };

    Ok(Json(ListSpecsResponse {
        specs: paged_specs,
        total,
        next_cursor,
        project_id: Some(project.id),
        hierarchy,
    }))
}

/// GET /api/projects/:projectId/specs/:spec - Get a spec within a project
pub async fn get_project_spec(
    State(state): State<AppState>,
    Path((project_id, spec_id)): Path<(String, String)>,
    headers: HeaderMap,
) -> ApiResult<Json<SpecDetail>> {
    if let Some(machine_id) = machine_id_from_headers(&headers) {
        let sync_state = state.sync_state.read().await;
        let machine = sync_state
            .persistent
            .machines
            .get(&machine_id)
            .ok_or_else(|| {
                (
                    StatusCode::NOT_FOUND,
                    Json(ApiError::invalid_request("Machine not found")),
                )
            })?;

        let project = machine.projects.get(&project_id).ok_or_else(|| {
            (
                StatusCode::NOT_FOUND,
                Json(ApiError::project_not_found(&project_id)),
            )
        })?;

        let record = project.specs.get(&spec_id).ok_or_else(|| {
            (
                StatusCode::NOT_FOUND,
                Json(ApiError::spec_not_found(&spec_id)),
            )
        })?;

        let mut detail = detail_from_record(&project.id, record);

        // Compute required_by (filter out self-references)
        let required_by = project
            .specs
            .values()
            .filter(|spec| {
                // Exclude the current spec itself AND check if it's in depends_on
                spec.spec_name != record.spec_name && spec.depends_on.contains(&record.spec_name)
            })
            .map(|spec| spec.spec_name.clone())
            .collect::<Vec<_>>();

        let children = project
            .specs
            .values()
            .filter(|spec| spec.parent.as_deref() == Some(record.spec_name.as_str()))
            .map(|spec| spec.spec_name.clone())
            .collect::<Vec<_>>();

        detail.required_by = required_by.clone();
        detail.children = children;
        detail.relationships = Some(crate::types::SpecRelationships {
            depends_on: detail.depends_on.clone(),
            required_by: Some(required_by),
        });

        return Ok(Json(detail));
    }

    let (loader, project) = get_spec_loader(&state, &project_id).await?;

    let spec = loader.load(&spec_id).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiError::internal_error(&e.to_string())),
        )
    })?;

    let spec = spec.ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            Json(ApiError::spec_not_found(&spec_id)),
        )
    })?;

    // Get dependency graph to compute required_by
    let all_specs = loader.load_all().unwrap_or_default();
    let dep_graph = DependencyGraph::new(&all_specs);

    let mut detail = SpecDetail::from(&spec).with_project_id(project.id.clone());

    // Compute required_by (filter out self-references)
    if let Some(complete) = dep_graph.get_complete_graph(&spec.path) {
        let required_by: Vec<String> = complete
            .required_by
            .iter()
            .filter(|s| s.path != spec.path) // Prevent self-reference bug
            .map(|s| s.path.clone())
            .collect();
        detail.required_by = required_by.clone();
        detail.relationships = Some(crate::types::SpecRelationships {
            depends_on: detail.depends_on.clone(),
            required_by: Some(required_by),
        });
    }

    let children: Vec<String> = all_specs
        .iter()
        .filter(|s| s.frontmatter.parent.as_deref() == Some(spec.path.as_str()))
        .map(|s| s.path.clone())
        .collect();
    detail.children = children;

    let sub_specs = detect_sub_specs(&detail.file_path);
    if !sub_specs.is_empty() {
        detail.sub_specs = Some(sub_specs);
    }

    Ok(Json(detail))
}

/// GET /api/projects/:projectId/specs/:spec/raw - Get raw spec content
pub async fn get_project_spec_raw(
    State(state): State<AppState>,
    Path((project_id, spec_id)): Path<(String, String)>,
    headers: HeaderMap,
) -> ApiResult<Json<SpecRawResponse>> {
    if machine_id_from_headers(&headers).is_some() {
        return Err((
            StatusCode::FORBIDDEN,
            Json(ApiError::invalid_request(
                "Raw spec access not supported for synced machines",
            )),
        ));
    }

    let (loader, _project) = get_spec_loader(&state, &project_id).await?;
    let spec = loader.load(&spec_id).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiError::internal_error(&e.to_string())),
        )
    })?;

    let spec = spec.ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            Json(ApiError::spec_not_found(&spec_id)),
        )
    })?;

    let content = fs::read_to_string(&spec.file_path).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiError::internal_error(&e.to_string())),
        )
    })?;
    let content_hash = hash_raw_content(&content);

    Ok(Json(SpecRawResponse {
        content,
        content_hash,
        file_path: spec.file_path.to_string_lossy().to_string(),
    }))
}

/// GET /api/projects/:projectId/specs/:spec/subspecs/:file/raw - Get raw sub-spec content
pub async fn get_project_subspec_raw(
    State(state): State<AppState>,
    Path((project_id, spec_id, file)): Path<(String, String, String)>,
    headers: HeaderMap,
) -> ApiResult<Json<SpecRawResponse>> {
    if machine_id_from_headers(&headers).is_some() {
        return Err((
            StatusCode::FORBIDDEN,
            Json(ApiError::invalid_request(
                "Raw sub-spec access not supported for synced machines",
            )),
        ));
    }

    if file.contains('/') || file.contains('\\') {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ApiError::invalid_request("Invalid sub-spec file")),
        ));
    }

    let (loader, _project) = get_spec_loader(&state, &project_id).await?;
    let spec = loader.load(&spec_id).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiError::internal_error(&e.to_string())),
        )
    })?;

    let spec = spec.ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            Json(ApiError::spec_not_found(&spec_id)),
        )
    })?;

    let parent_dir = spec.file_path.parent().ok_or_else(|| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiError::internal_error("Missing spec directory")),
        )
    })?;
    let file_path = parent_dir.join(&file);
    if !file_path.exists() {
        return Err((
            StatusCode::NOT_FOUND,
            Json(ApiError::invalid_request("Sub-spec not found")),
        ));
    }

    let content = fs::read_to_string(&file_path).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiError::internal_error(&e.to_string())),
        )
    })?;
    let content_hash = hash_raw_content(&content);

    Ok(Json(SpecRawResponse {
        content,
        content_hash,
        file_path: file_path.to_string_lossy().to_string(),
    }))
}

/// POST /api/projects/:projectId/search - Search specs in a project
pub async fn search_project_specs(
    State(state): State<AppState>,
    Path(project_id): Path<String>,
    headers: HeaderMap,
    Json(req): Json<SearchRequest>,
) -> ApiResult<Json<SearchResponse>> {
    if let Some(machine_id) = machine_id_from_headers(&headers) {
        let sync_state = state.sync_state.read().await;
        let machine = sync_state
            .persistent
            .machines
            .get(&machine_id)
            .ok_or_else(|| {
                (
                    StatusCode::NOT_FOUND,
                    Json(ApiError::invalid_request("Machine not found")),
                )
            })?;

        let project = machine.projects.get(&project_id).ok_or_else(|| {
            (
                StatusCode::NOT_FOUND,
                Json(ApiError::project_not_found(&project_id)),
            )
        })?;

        let query_lower = req.query.to_lowercase();
        let mut results: Vec<SpecSummary> = project
            .specs
            .values()
            .filter(|spec| {
                spec.spec_name.to_lowercase().contains(&query_lower)
                    || spec
                        .title
                        .as_ref()
                        .map(|title| title.to_lowercase().contains(&query_lower))
                        .unwrap_or(false)
                    || spec.content_md.to_lowercase().contains(&query_lower)
                    || spec
                        .tags
                        .iter()
                        .any(|tag| tag.to_lowercase().contains(&query_lower))
            })
            .filter(|spec| {
                if let Some(ref filters) = req.filters {
                    if let Some(ref status) = filters.status {
                        if &spec.status != status {
                            return false;
                        }
                    }
                    if let Some(ref priority) = filters.priority {
                        match &spec.priority {
                            Some(p) if p == priority => {}
                            _ => return false,
                        }
                    }
                    if let Some(ref tags) = filters.tags {
                        if !tags.iter().all(|t| spec.tags.contains(t)) {
                            return false;
                        }
                    }
                }
                true
            })
            .map(|spec| summary_from_record(&project.id, spec))
            .collect();

        results.sort_by(|a, b| {
            let a_title_match = a
                .title
                .as_ref()
                .map(|t| t.to_lowercase().contains(&query_lower))
                .unwrap_or(false);
            let b_title_match = b
                .title
                .as_ref()
                .map(|t| t.to_lowercase().contains(&query_lower))
                .unwrap_or(false);

            match (a_title_match, b_title_match) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => b.spec_number.cmp(&a.spec_number),
            }
        });

        let total = results.len();
        return Ok(Json(SearchResponse {
            results,
            total,
            query: req.query,
            project_id: Some(project.id.clone()),
        }));
    }

    let (loader, project) = get_spec_loader(&state, &project_id).await?;

    let all_specs = loader.load_all().map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiError::internal_error(&e.to_string())),
        )
    })?;

    // Build enriched query by appending UI filter fields to the search query.
    // This lets the core search engine handle field filters (status:, priority:, tag:)
    // alongside the user's text query with boolean operators, fuzzy matching, etc.
    let mut enriched_query = req.query.clone();
    if let Some(ref filters) = req.filters {
        if let Some(ref status) = filters.status {
            enriched_query.push_str(&format!(" status:{}", status));
        }
        if let Some(ref priority) = filters.priority {
            enriched_query.push_str(&format!(" priority:{}", priority));
        }
        if let Some(ref tags) = filters.tags {
            for tag in tags {
                enriched_query.push_str(&format!(" tag:{}", tag));
            }
        }
    }

    // Use the core search engine for advanced query parsing, fuzzy matching,
    // boolean operators, field filters, and relevance scoring.
    let search_results =
        search_specs_with_options(&all_specs, &enriched_query, SearchOptions::new());

    // Map core SearchResults back to SpecSummary objects.
    // Build a lookup from path → SpecInfo for efficient mapping.
    let spec_map: std::collections::HashMap<&str, &harnspec_core::SpecInfo> =
        all_specs.iter().map(|s| (s.path.as_str(), s)).collect();

    let results: Vec<SpecSummary> = search_results
        .iter()
        .filter_map(|sr| {
            spec_map
                .get(sr.path.as_str())
                .map(|s| SpecSummary::from(*s).with_project_id(&project.id))
        })
        .collect();

    let total = results.len();

    Ok(Json(SearchResponse {
        results,
        total,
        query: req.query,
        project_id: Some(project.id),
    }))
}
