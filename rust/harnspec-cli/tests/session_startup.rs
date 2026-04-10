mod common;
use common::*;
use tempfile::TempDir;

#[test]
fn test_session_create_no_project_path_defaults_to_cwd() {
    let ctx = TestContext::new();
    let home = TempDir::new().expect("Failed to create home directory");
    let cwd = ctx.path();

    // 1. Initialize project and create a spec
    init_project(cwd, true);
    create_spec(cwd, "test-spec");

    // 2. Setup a test runner
    write_test_runner(cwd, "test-runner");

    // 3. Create a session WITHOUT --project-path
    // We use exec_cli_env because session_create helper in common/mod.rs
    // force-adds --project-path.
    let args = vec![
        "session",
        "create",
        "--runner",
        "test-runner",
        "--spec",
        "test-spec",
    ];
    let result = exec_cli_env(&args, cwd, &[("HOME", home.path().to_str().unwrap())]);

    assert!(
        result.success,
        "session create should succeed without project-path: {}",
        result.stderr
    );
    assert!(
        result.stdout.contains("Created session"),
        "should mention session creation"
    );

    let session_id = parse_session_id(&result.stdout).expect("should have a session ID");
    assert_eq!(session_id.len(), 36, "session ID should be a UUID");
}

#[test]
fn test_run_direct_no_project_path_defaults_to_cwd() {
    let ctx = TestContext::new();
    let home = TempDir::new().expect("Failed to create home directory");
    let cwd = ctx.path();

    // 1. Initialize project and create a spec
    init_project(cwd, true);
    create_spec(cwd, "test-spec");

    // 2. Setup a test runner
    write_test_runner(cwd, "test-runner");

    // 3. Run directly WITHOUT --project-path
    let args = vec![
        "run",
        "--runner",
        "test-runner",
        "--spec",
        "test-spec",
        "--dry-run",
    ];
    let result = exec_cli_env(&args, cwd, &[("HOME", home.path().to_str().unwrap())]);

    assert!(
        result.success,
        "run direct should succeed without project-path: {}",
        result.stderr
    );
    assert!(
        result.stdout.contains("Dry run"),
        "should show dry run output"
    );
    assert!(
        result.stdout.contains("Runner: test-runner"),
        "should use the correct runner"
    );
}
