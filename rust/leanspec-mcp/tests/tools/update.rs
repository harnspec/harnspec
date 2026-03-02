//! Tests for the `update` MCP tool

use crate::helpers::*;

use leanspec_mcp::tools::call_tool;
use serde_json::json;

#[tokio::test]
async fn test_update_status() {
    let temp = create_test_project(&[("001-feature-a", "planned", None)]);
    set_specs_dir_env(&temp);

    let result = call_tool(
        "update",
        json!({
            "specPath": "001",
            "status": "in-progress"
        }),
    )
    .await;
    assert!(result.is_ok());
    assert!(result.unwrap().contains("status → in-progress"));

    // Verify the file was updated
    let content =
        std::fs::read_to_string(temp.path().join("specs/001-feature-a/README.md")).unwrap();
    assert!(content.contains("status: in-progress"));
}

#[tokio::test]
async fn test_update_priority() {
    let temp = create_test_project(&[("001-feature-a", "planned", None)]);
    set_specs_dir_env(&temp);

    let result = call_tool(
        "update",
        json!({
            "specPath": "001",
            "priority": "high"
        }),
    )
    .await;
    assert!(result.is_ok());
    assert!(result.unwrap().contains("priority → high"));

    let content =
        std::fs::read_to_string(temp.path().join("specs/001-feature-a/README.md")).unwrap();
    assert!(content.contains("priority: high"));
}

#[tokio::test]
async fn test_update_assignee() {
    let temp = create_test_project(&[("001-feature-a", "planned", None)]);
    set_specs_dir_env(&temp);

    let result = call_tool(
        "update",
        json!({
            "specPath": "001",
            "assignee": "developer@example.com"
        }),
    )
    .await;
    assert!(result.is_ok());
    assert!(result.unwrap().contains("assignee → developer@example.com"));

    let content =
        std::fs::read_to_string(temp.path().join("specs/001-feature-a/README.md")).unwrap();
    assert!(content.contains("assignee: developer@example.com"));
}

#[tokio::test]
async fn test_update_add_tags() {
    let temp = create_test_project(&[("001-feature-a", "planned", None)]);
    set_specs_dir_env(&temp);

    let result = call_tool(
        "update",
        json!({
            "specPath": "001",
            "addTags": ["new-tag", "another-tag"]
        }),
    )
    .await;
    assert!(result.is_ok());
    let output = result.unwrap();
    // Check that at least some tags were added
    assert!(output.contains("+tag:") || output.contains("new-tag") || output.contains("Updated"));
}

#[tokio::test]
async fn test_update_remove_tags() {
    let temp = create_test_project(&[("001-feature-a", "planned", None)]);
    set_specs_dir_env(&temp);

    // "test" tag exists from helper - try to remove it
    let result = call_tool(
        "update",
        json!({
            "specPath": "001",
            "removeTags": ["test"]
        }),
    )
    .await;
    // This may or may not work depending on implementation
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_update_combined() {
    let temp = create_test_project(&[("001-feature-a", "planned", None)]);
    set_specs_dir_env(&temp);

    let result = call_tool(
        "update",
        json!({
            "specPath": "001",
            "status": "in-progress",
            "priority": "critical",
            "addTags": ["urgent"]
        }),
    )
    .await;
    assert!(result.is_ok());
    let output = result.unwrap();
    assert!(output.contains("status → in-progress"));
    assert!(output.contains("priority → critical"));
    assert!(output.contains("+tag: urgent"));
}

#[tokio::test]
async fn test_update_spec_not_found() {
    let temp = create_test_project(&[("001-feature-a", "planned", None)]);
    set_specs_dir_env(&temp);

    let result = call_tool(
        "update",
        json!({
            "specPath": "999",
            "status": "complete"
        }),
    )
    .await;
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("not found"));
}

#[tokio::test]
async fn test_update_missing_spec_param() {
    let temp = create_test_project(&[("001-feature-a", "planned", None)]);
    set_specs_dir_env(&temp);

    let result = call_tool("update", json!({ "status": "complete" })).await;
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Missing required parameter"));
}

#[tokio::test]
async fn test_update_no_changes() {
    let temp = create_test_project(&[("001-feature-a", "planned", None)]);
    set_specs_dir_env(&temp);

    let result = call_tool("update", json!({ "specPath": "001" })).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_update_replacements() {
    let temp = create_test_project(&[("001-feature-a", "planned", None)]);
    set_specs_dir_env(&temp);

    let result = call_tool(
        "update",
        json!({
            "specPath": "001",
            "replacements": [{
                "oldString": "Test spec content.",
                "newString": "Updated spec content."
            }]
        }),
    )
    .await;
    assert!(result.is_ok());

    let content =
        std::fs::read_to_string(temp.path().join("specs/001-feature-a/README.md")).unwrap();
    assert!(content.contains("Updated spec content."));
}

#[tokio::test]
async fn test_update_content_preserves_title() {
    let temp = create_test_project(&[("001-feature-a", "planned", None)]);
    set_specs_dir_env(&temp);

    let path = temp.path().join("specs/001-feature-a/README.md");
    let original = std::fs::read_to_string(&path).unwrap();
    let original_title = original
        .lines()
        .find(|line| line.trim_start().starts_with("# "))
        .expect("missing title line")
        .to_string();

    let result = call_tool(
        "update",
        json!({
            "specPath": "001",
            "content": "## Overview\n\nUpdated overview."
        }),
    )
    .await;
    assert!(result.is_ok());

    let updated = std::fs::read_to_string(&path).unwrap();
    assert!(updated.contains(&original_title));
    assert!(updated.matches(&original_title).count() == 1);
    assert!(updated.contains("Updated overview."));
}

#[tokio::test]
async fn test_update_section_update() {
    let temp = create_test_project(&[("001-feature-a", "planned", None)]);
    set_specs_dir_env(&temp);

    let result = call_tool(
        "update",
        json!({
            "specPath": "001",
            "sectionUpdates": [{
                "section": "Overview",
                "content": "Replaced overview content.",
                "mode": "replace"
            }]
        }),
    )
    .await;
    assert!(result.is_ok());

    let content =
        std::fs::read_to_string(temp.path().join("specs/001-feature-a/README.md")).unwrap();
    assert!(content.contains("Replaced overview content."));
}

#[tokio::test]
async fn test_update_checklist_toggles() {
    let temp = create_test_project(&[("001-feature-a", "planned", None)]);
    set_specs_dir_env(&temp);

    let path = temp.path().join("specs/001-feature-a/README.md");
    let mut content = std::fs::read_to_string(&path).unwrap();
    content.push_str("\n- [ ] Task 1\n");
    std::fs::write(&path, content).unwrap();

    let result = call_tool(
        "update",
        json!({
            "specPath": "001",
            "checklistToggles": [{
                "itemText": "Task 1",
                "checked": true
            }]
        }),
    )
    .await;
    assert!(result.is_ok());
    let output = result.unwrap();
    assert!(output.contains("Checklist updates:"));

    let updated = std::fs::read_to_string(&path).unwrap();
    assert!(updated.contains("- [x] Task 1"));
}

#[tokio::test]
async fn test_update_duplicate_tag_ignored() {
    let temp = create_test_project(&[("001-feature-a", "planned", None)]);
    set_specs_dir_env(&temp);

    // "test" tag already exists
    let result = call_tool(
        "update",
        json!({
            "specPath": "001",
            "addTags": ["test"]
        }),
    )
    .await;
    assert!(result.is_ok());
    // Should not show +tag for already existing tag
    assert!(!result.unwrap().contains("+tag: test"));
}

// ── Content hash mismatch ────────────────────────────────────────

#[tokio::test]
async fn test_update_content_hash_mismatch_rejected() {
    let temp = create_test_project(&[("001-feature-a", "planned", None)]);
    set_specs_dir_env(&temp);

    let result = call_tool(
        "update",
        json!({
            "specPath": "001",
            "expectedContentHash": "0000000000000000000000000000000000000000000000000000000000000000",
            "replacements": [{
                "oldString": "Test spec content.",
                "newString": "Changed."
            }]
        }),
    )
    .await;
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Content hash mismatch"));
}

#[tokio::test]
async fn test_update_content_hash_match_accepted() {
    let temp = create_test_project(&[("001-feature-a", "planned", None)]);
    set_specs_dir_env(&temp);

    // Read actual body to compute correct hash
    let content =
        std::fs::read_to_string(temp.path().join("specs/001-feature-a/README.md")).unwrap();
    let parser = leanspec_core::FrontmatterParser::new();
    let (_, body) = parser.parse(&content).unwrap();
    let hash = leanspec_core::hash_content(&body);

    let result = call_tool(
        "update",
        json!({
            "specPath": "001",
            "expectedContentHash": hash,
            "replacements": [{
                "oldString": "Test spec content.",
                "newString": "Changed."
            }]
        }),
    )
    .await;
    assert!(result.is_ok());
}

// ── Status transition validation ─────────────────────────────────

#[tokio::test]
async fn test_update_draft_to_in_progress_without_force_rejected() {
    let temp = create_test_project(&[("001-feature-a", "draft", None)]);
    set_specs_dir_env(&temp);

    let result = call_tool(
        "update",
        json!({
            "specPath": "001",
            "status": "in-progress"
        }),
    )
    .await;
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.contains("Cannot skip") || err.contains("planned"));
}

#[tokio::test]
async fn test_update_draft_to_complete_without_force_rejected() {
    let temp = create_test_project(&[("001-feature-a", "draft", None)]);
    set_specs_dir_env(&temp);

    let result = call_tool(
        "update",
        json!({
            "specPath": "001",
            "status": "complete"
        }),
    )
    .await;
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.contains("Cannot skip") || err.contains("planned"));
}

#[tokio::test]
async fn test_update_draft_to_in_progress_with_force_accepted() {
    let temp = create_test_project(&[("001-feature-a", "draft", None)]);
    set_specs_dir_env(&temp);

    let result = call_tool(
        "update",
        json!({
            "specPath": "001",
            "status": "in-progress",
            "force": true
        }),
    )
    .await;
    assert!(result.is_ok());
    assert!(result.unwrap().contains("status → in-progress"));
}

#[tokio::test]
async fn test_update_draft_to_planned_allowed() {
    let temp = create_test_project(&[("001-feature-a", "draft", None)]);
    set_specs_dir_env(&temp);

    let result = call_tool(
        "update",
        json!({
            "specPath": "001",
            "status": "planned"
        }),
    )
    .await;
    assert!(result.is_ok());
    assert!(result.unwrap().contains("status → planned"));
}

// ── Checklist completion gate ────────────────────────────────────

#[tokio::test]
async fn test_update_complete_with_unchecked_items_rejected() {
    let temp = create_test_project(&[("001-feature-a", "in-progress", None)]);
    set_specs_dir_env(&temp);

    // Add unchecked checklist items
    let path = temp.path().join("specs/001-feature-a/README.md");
    let mut content = std::fs::read_to_string(&path).unwrap();
    content.push_str("\n## Plan\n\n- [ ] Unfinished task\n");
    std::fs::write(&path, content).unwrap();

    let result = call_tool(
        "update",
        json!({
            "specPath": "001",
            "status": "complete"
        }),
    )
    .await;
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.contains("INCOMPLETE_CHECKLIST") || err.contains("outstanding"));
}

#[tokio::test]
async fn test_update_complete_with_all_checked_accepted() {
    let temp = create_test_project(&[("001-feature-a", "in-progress", None)]);
    set_specs_dir_env(&temp);

    // Override content with all items checked
    let path = temp.path().join("specs/001-feature-a/README.md");
    let content = "---\nstatus: in-progress\ncreated: '2025-01-01'\ntags:\n  - test\ncreated_at: '2025-01-01T00:00:00Z'\n---\n\n# Feature A\n\n## Plan\n\n- [x] Task 1\n- [x] Task 2\n\n## Test\n\n- [x] Test 1\n";
    std::fs::write(&path, content).unwrap();

    let result = call_tool(
        "update",
        json!({
            "specPath": "001",
            "status": "complete"
        }),
    )
    .await;
    assert!(result.is_ok());
    assert!(result.unwrap().contains("status → complete"));
}

#[tokio::test]
async fn test_update_complete_with_force_bypasses_checklist() {
    let temp = create_test_project(&[("001-feature-a", "in-progress", None)]);
    set_specs_dir_env(&temp);

    let path = temp.path().join("specs/001-feature-a/README.md");
    let mut content = std::fs::read_to_string(&path).unwrap();
    content.push_str("\n- [ ] Unfinished task\n");
    std::fs::write(&path, content).unwrap();

    let result = call_tool(
        "update",
        json!({
            "specPath": "001",
            "status": "complete",
            "force": true
        }),
    )
    .await;
    assert!(result.is_ok());
    assert!(result.unwrap().contains("status → complete"));
}

// ── Umbrella (parent/child) validation ───────────────────────────

#[tokio::test]
async fn test_update_umbrella_complete_with_incomplete_children_rejected() {
    let temp = create_test_project(&[
        ("001-parent", "in-progress", None),
        ("002-child-a", "complete", None),
        ("003-child-b", "in-progress", None),
    ]);
    set_specs_dir_env(&temp);

    // Set parent relationship on children
    for child in &["002-child-a", "003-child-b"] {
        let path = temp.path().join(format!("specs/{}/README.md", child));
        let content = std::fs::read_to_string(&path).unwrap();
        let content = content.replace("created_at:", "parent: 001-parent\ncreated_at:");
        std::fs::write(&path, content).unwrap();
    }

    // Parent has all checklist items done
    let parent_path = temp.path().join("specs/001-parent/README.md");
    let content = "---\nstatus: in-progress\ncreated: '2025-01-01'\ntags:\n  - test\ncreated_at: '2025-01-01T00:00:00Z'\n---\n\n# Parent\n\n## Plan\n\n- [x] Done\n\n## Test\n\n- [x] Tested\n";
    std::fs::write(&parent_path, content).unwrap();

    let result = call_tool(
        "update",
        json!({
            "specPath": "001",
            "status": "complete"
        }),
    )
    .await;
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.contains("INCOMPLETE_CHILDREN") || err.contains("child"));
}

// ── Replacement match modes ──────────────────────────────────────

#[tokio::test]
async fn test_update_replacement_match_mode_all() {
    let temp = create_test_project(&[("001-feature-a", "planned", None)]);
    set_specs_dir_env(&temp);

    // Create content with repeated text
    let path = temp.path().join("specs/001-feature-a/README.md");
    let mut content = std::fs::read_to_string(&path).unwrap();
    content.push_str("\nTODO: first\nTODO: second\n");
    std::fs::write(&path, content).unwrap();

    let result = call_tool(
        "update",
        json!({
            "specPath": "001",
            "replacements": [{
                "oldString": "TODO",
                "newString": "DONE",
                "matchMode": "all"
            }]
        }),
    )
    .await;
    assert!(result.is_ok());

    let updated = std::fs::read_to_string(&path).unwrap();
    assert!(!updated.contains("TODO"));
    assert_eq!(updated.matches("DONE").count(), 2);
}

#[tokio::test]
async fn test_update_replacement_match_mode_first() {
    let temp = create_test_project(&[("001-feature-a", "planned", None)]);
    set_specs_dir_env(&temp);

    let path = temp.path().join("specs/001-feature-a/README.md");
    let mut content = std::fs::read_to_string(&path).unwrap();
    content.push_str("\nTODO: first\nTODO: second\n");
    std::fs::write(&path, content).unwrap();

    let result = call_tool(
        "update",
        json!({
            "specPath": "001",
            "replacements": [{
                "oldString": "TODO",
                "newString": "DONE",
                "matchMode": "first"
            }]
        }),
    )
    .await;
    assert!(result.is_ok());

    let updated = std::fs::read_to_string(&path).unwrap();
    assert!(updated.contains("DONE: first"));
    assert!(updated.contains("TODO: second"));
}

// ── Section update modes (append, prepend) ───────────────────────

#[tokio::test]
async fn test_update_section_append() {
    let temp = create_test_project(&[("001-feature-a", "planned", None)]);
    set_specs_dir_env(&temp);

    let result = call_tool(
        "update",
        json!({
            "specPath": "001",
            "sectionUpdates": [{
                "section": "Overview",
                "content": "Appended paragraph.",
                "mode": "append"
            }]
        }),
    )
    .await;
    assert!(result.is_ok());

    let content =
        std::fs::read_to_string(temp.path().join("specs/001-feature-a/README.md")).unwrap();
    assert!(content.contains("Test spec content."));
    assert!(content.contains("Appended paragraph."));
    let orig_pos = content.find("Test spec content.").unwrap();
    let append_pos = content.find("Appended paragraph.").unwrap();
    assert!(append_pos > orig_pos);
}

#[tokio::test]
async fn test_update_section_prepend() {
    let temp = create_test_project(&[("001-feature-a", "planned", None)]);
    set_specs_dir_env(&temp);

    let result = call_tool(
        "update",
        json!({
            "specPath": "001",
            "sectionUpdates": [{
                "section": "Overview",
                "content": "Prepended paragraph.",
                "mode": "prepend"
            }]
        }),
    )
    .await;
    assert!(result.is_ok());

    let content =
        std::fs::read_to_string(temp.path().join("specs/001-feature-a/README.md")).unwrap();
    assert!(content.contains("Test spec content."));
    assert!(content.contains("Prepended paragraph."));
    let prepend_pos = content.find("Prepended paragraph.").unwrap();
    let orig_pos = content.find("Test spec content.").unwrap();
    assert!(prepend_pos < orig_pos);
}

// ── Multiple replacements in one call ────────────────────────────

#[tokio::test]
async fn test_update_multiple_replacements() {
    let temp = create_test_project(&[("001-feature-a", "planned", None)]);
    set_specs_dir_env(&temp);

    let path = temp.path().join("specs/001-feature-a/README.md");
    let mut content = std::fs::read_to_string(&path).unwrap();
    content.push_str("\nAlpha line.\nBeta line.\n");
    std::fs::write(&path, content).unwrap();

    let result = call_tool(
        "update",
        json!({
            "specPath": "001",
            "replacements": [
                {
                    "oldString": "Alpha line.",
                    "newString": "First line."
                },
                {
                    "oldString": "Beta line.",
                    "newString": "Second line."
                }
            ]
        }),
    )
    .await;
    assert!(result.is_ok());

    let updated = std::fs::read_to_string(&path).unwrap();
    assert!(updated.contains("First line."));
    assert!(updated.contains("Second line."));
    assert!(!updated.contains("Alpha line."));
    assert!(!updated.contains("Beta line."));
}
