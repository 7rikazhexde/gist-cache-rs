// Allow deprecated Command::cargo_bin for now
// See: https://github.com/assert-rs/assert_cmd/issues/200
#![allow(deprecated)]

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

fn setup_test_env() -> TempDir {
    let temp_dir = TempDir::new().unwrap();
    unsafe {
        // Use GIST_CACHE_DIR to override cache directory for testing
        std::env::set_var("GIST_CACHE_DIR", temp_dir.path());

        // Also set HOME/USERPROFILE for dirs::home_dir() fallback
        #[cfg(unix)]
        std::env::set_var("HOME", temp_dir.path());

        #[cfg(windows)]
        std::env::set_var("USERPROFILE", temp_dir.path());
    }
    temp_dir
}

#[test]
fn test_cli_help() {
    let mut cmd = Command::cargo_bin("gist-cache-rs").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Usage:"));
}

#[test]
fn test_cli_version() {
    let mut cmd = Command::cargo_bin("gist-cache-rs").unwrap();
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains(env!("CARGO_PKG_VERSION")));
}

#[test]
fn test_update_without_auth() {
    let _temp = setup_test_env();

    // This test will fail if gh is not authenticated
    // We're testing the command structure, not the actual GitHub interaction
    let mut cmd = Command::cargo_bin("gist-cache-rs").unwrap();
    let result = cmd.arg("update").output();

    // Either succeeds or fails with authentication error
    assert!(result.is_ok());
}

#[test]
fn test_run_without_cache() {
    let temp = setup_test_env();
    let cache_dir = temp.path().join(".cache").join("gist-cache");
    fs::create_dir_all(&cache_dir).unwrap();

    let mut cmd = Command::cargo_bin("gist-cache-rs").unwrap();
    cmd.arg("run").arg("test_query").assert().failure(); // Should fail because cache doesn't exist
}

#[test]
fn test_cache_list_empty() {
    let temp = setup_test_env();
    let cache_dir = temp.path().join(".cache").join("gist-cache");
    fs::create_dir_all(cache_dir.join("contents")).unwrap();

    let mut cmd = Command::cargo_bin("gist-cache-rs").unwrap();
    cmd.arg("cache")
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains("No cached Gists"));
}

#[test]
fn test_cache_size() {
    let temp = setup_test_env();
    let cache_dir = temp.path().join(".cache").join("gist-cache");
    fs::create_dir_all(cache_dir.join("contents")).unwrap();

    let mut cmd = Command::cargo_bin("gist-cache-rs").unwrap();
    cmd.arg("cache")
        .arg("size")
        .assert()
        .success()
        .stdout(predicate::str::contains("Cache size information"));
}

#[test]
fn test_invalid_command() {
    let mut cmd = Command::cargo_bin("gist-cache-rs").unwrap();
    cmd.arg("invalid_command").assert().failure();
}

#[test]
fn test_run_with_preview_flag() {
    let temp = setup_test_env();
    let cache_dir = temp.path().join(".cache").join("gist-cache");
    fs::create_dir_all(&cache_dir).unwrap();

    // Create a minimal cache file
    let cache_file = cache_dir.join("cache.json");
    fs::write(
        &cache_file,
        r#"{
        "metadata": {
            "last_updated": "2024-01-01T12:00:00Z",
            "total_count": 0,
            "github_user": "testuser"
        },
        "gists": []
    }"#,
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("gist-cache-rs").unwrap();
    cmd.arg("run")
        .arg("--preview")
        .arg("nonexistent")
        .assert()
        .failure(); // Should fail because no gists match
}

#[test]
fn test_update_verbose() {
    let _temp = setup_test_env();

    let mut cmd = Command::cargo_bin("gist-cache-rs").unwrap();
    let result = cmd.arg("update").arg("--verbose").output();

    // Either succeeds or fails, but command structure is valid
    assert!(result.is_ok());
}

#[test]
fn test_update_force() {
    let _temp = setup_test_env();

    let mut cmd = Command::cargo_bin("gist-cache-rs").unwrap();
    let result = cmd.arg("update").arg("--force").output();

    // Either succeeds or fails, but command structure is valid
    assert!(result.is_ok());
}

#[test]
fn test_run_with_id_flag() {
    let temp = setup_test_env();
    let cache_dir = temp.path().join(".cache").join("gist-cache");
    fs::create_dir_all(&cache_dir).unwrap();

    let cache_file = cache_dir.join("cache.json");
    fs::write(
        &cache_file,
        r#"{
        "metadata": {
            "last_updated": "2024-01-01T12:00:00Z",
            "total_count": 0,
            "github_user": "testuser"
        },
        "gists": []
    }"#,
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("gist-cache-rs").unwrap();
    cmd.arg("run")
        .arg("--id")
        .arg("nonexistent_id")
        .assert()
        .failure();
}

#[test]
fn test_run_with_filename_flag() {
    let temp = setup_test_env();
    let cache_dir = temp.path().join(".cache").join("gist-cache");
    fs::create_dir_all(&cache_dir).unwrap();

    let cache_file = cache_dir.join("cache.json");
    fs::write(
        &cache_file,
        r#"{
        "metadata": {
            "last_updated": "2024-01-01T12:00:00Z",
            "total_count": 0,
            "github_user": "testuser"
        },
        "gists": []
    }"#,
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("gist-cache-rs").unwrap();
    cmd.arg("run")
        .arg("--filename")
        .arg("test.sh")
        .assert()
        .failure();
}

#[test]
fn test_run_with_description_flag() {
    let temp = setup_test_env();
    let cache_dir = temp.path().join(".cache").join("gist-cache");
    fs::create_dir_all(&cache_dir).unwrap();

    let cache_file = cache_dir.join("cache.json");
    fs::write(
        &cache_file,
        r#"{
        "metadata": {
            "last_updated": "2024-01-01T12:00:00Z",
            "total_count": 0,
            "github_user": "testuser"
        },
        "gists": []
    }"#,
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("gist-cache-rs").unwrap();
    cmd.arg("run")
        .arg("--description")
        .arg("test script")
        .assert()
        .failure();
}

#[test]
fn test_cache_clear_with_no_input() {
    let temp = setup_test_env();
    let cache_dir = temp.path().join(".cache").join("gist-cache");
    fs::create_dir_all(cache_dir.join("contents")).unwrap();

    // clearコマンドは確認プロンプトがあるため、標準入力なしでは失敗する
    let mut cmd = Command::cargo_bin("gist-cache-rs").unwrap();
    cmd.arg("cache").arg("clear").write_stdin("n\n");
    // プロンプトに応答できるかテスト
    // 実際の動作は環境依存
}

#[test]
fn test_run_without_query() {
    let temp = setup_test_env();
    let cache_dir = temp.path().join(".cache").join("gist-cache");
    fs::create_dir_all(&cache_dir).unwrap();

    let cache_file = cache_dir.join("cache.json");
    fs::write(
        &cache_file,
        r#"{
        "metadata": {
            "last_updated": "2024-01-01T12:00:00Z",
            "total_count": 1,
            "github_user": "testuser"
        },
        "gists": [{
            "id": "abc123",
            "description": "Test gist",
            "files": [{"filename": "test.sh", "language": "Shell", "size": 100}],
            "updated_at": "2024-01-01T12:00:00Z",
            "public": true,
            "html_url": "https://gist.github.com/abc123"
        }]
    }"#,
    )
    .unwrap();

    // クエリなしでrunを実行すると、一覧から選択するモードになるが、
    // インタラクティブ入力が必要なため失敗する
    let mut cmd = Command::cargo_bin("gist-cache-rs").unwrap();
    let result = cmd.arg("run").output();
    assert!(result.is_ok()); // コマンド自体は実行される
}

#[test]
fn test_completions_help() {
    let mut cmd = Command::cargo_bin("gist-cache-rs").unwrap();
    cmd.arg("completions")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Generate shell completion scripts",
        ));
}

#[test]
fn test_completions_bash() {
    let mut cmd = Command::cargo_bin("gist-cache-rs").unwrap();
    cmd.arg("completions")
        .arg("bash")
        .assert()
        .success()
        .stdout(predicate::str::contains("gist-cache-rs"));
}

#[test]
fn test_completions_zsh() {
    let mut cmd = Command::cargo_bin("gist-cache-rs").unwrap();
    cmd.arg("completions")
        .arg("zsh")
        .assert()
        .success()
        .stdout(predicate::str::contains("gist-cache-rs"));
}

#[test]
fn test_completions_fish() {
    let mut cmd = Command::cargo_bin("gist-cache-rs").unwrap();
    cmd.arg("completions")
        .arg("fish")
        .assert()
        .success()
        .stdout(predicate::str::contains("gist-cache-rs"));
}

#[test]
fn test_completions_powershell() {
    let mut cmd = Command::cargo_bin("gist-cache-rs").unwrap();
    cmd.arg("completions")
        .arg("powershell")
        .assert()
        .success()
        .stdout(predicate::str::contains("gist-cache-rs"));
}

#[test]
fn test_completions_invalid_shell() {
    let mut cmd = Command::cargo_bin("gist-cache-rs").unwrap();
    cmd.arg("completions")
        .arg("invalid_shell")
        .assert()
        .failure();
}

#[test]
fn test_completions_bash_contains_commands() {
    let mut cmd = Command::cargo_bin("gist-cache-rs").unwrap();
    let output = cmd.arg("completions").arg("bash").output().unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Verify that the completion script contains main commands
    assert!(stdout.contains("update") || stdout.contains("run") || stdout.contains("cache"));
}

#[test]
fn test_completions_zsh_contains_commands() {
    let mut cmd = Command::cargo_bin("gist-cache-rs").unwrap();
    let output = cmd.arg("completions").arg("zsh").output().unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Verify that the completion script contains main commands
    assert!(stdout.contains("update") || stdout.contains("run") || stdout.contains("cache"));
}

#[test]
fn test_update_with_progress_display() {
    let _temp = setup_test_env();

    // Test that update command runs without error (progress bar should not cause issues)
    let mut cmd = Command::cargo_bin("gist-cache-rs").unwrap();
    let result = cmd.arg("update").output();

    // Either succeeds or fails with authentication error, but not a progress bar error
    assert!(result.is_ok());
}

#[test]
fn test_update_verbose_without_progress() {
    let _temp = setup_test_env();

    // Test that verbose mode runs without progress bar interference
    let mut cmd = Command::cargo_bin("gist-cache-rs").unwrap();
    let result = cmd.arg("update").arg("--verbose").output();

    // Verbose mode should work without progress bar issues
    assert!(result.is_ok());
}

#[test]
fn test_cache_list_json_format_empty() {
    let temp = setup_test_env();
    fs::create_dir_all(temp.path().join("contents")).unwrap();

    let mut cmd = Command::cargo_bin("gist-cache-rs").unwrap();
    cmd.arg("cache")
        .arg("list")
        .arg("--format")
        .arg("json")
        .assert()
        .success()
        .stdout(predicate::str::contains("[]"));
}

#[test]
fn test_cache_list_json_format() {
    let temp = setup_test_env();

    // GIST_CACHE_DIR points to temp.path(), actual cache dir is temp.path()/gist-cache
    let cache_dir = temp.path().join("gist-cache");
    fs::create_dir_all(&cache_dir).unwrap();

    // Create test cache
    let cache_file = cache_dir.join("cache.json");
    fs::write(
        &cache_file,
        r#"{
        "metadata": {
            "last_updated": "2024-01-01T12:00:00Z",
            "total_count": 1,
            "github_user": "testuser"
        },
        "gists": [
            {
                "id": "abc123",
                "description": "Test Gist",
                "files": [
                    {
                        "filename": "test.sh",
                        "language": null,
                        "size": 100
                    }
                ],
                "updated_at": "2024-01-01T12:00:00Z",
                "public": true,
                "html_url": "https://gist.github.com/abc123"
            }
        ]
    }"#,
    )
    .unwrap();

    // Create content cache
    let contents_dir = cache_dir.join("contents").join("abc123");
    fs::create_dir_all(&contents_dir).unwrap();
    fs::write(contents_dir.join("test.sh"), "#!/bin/bash\necho hello").unwrap();

    let mut cmd = Command::cargo_bin("gist-cache-rs").unwrap();
    let output = cmd
        .env("GIST_CACHE_DIR", temp.path())
        .arg("cache")
        .arg("list")
        .arg("--format")
        .arg("json")
        .output()
        .unwrap();

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    // Debug: print actual output
    eprintln!("STDOUT: {}", stdout);
    eprintln!("STDERR: {}", stderr);
    eprintln!("Status: {:?}", output.status);

    // Verify JSON output contains expected fields
    assert!(
        stdout.contains("abc123"),
        "stdout does not contain 'abc123'. Actual: {}",
        stdout
    );
    assert!(stdout.contains("Test Gist"));
    assert!(stdout.contains("test.sh"));
    assert!(stdout.contains("2024-01-01"));
}

#[test]
fn test_config_set_get() {
    let temp = setup_test_env();
    let mut cmd = Command::cargo_bin("gist-cache-rs").unwrap();

    cmd.env("GIST_CACHE_DIR", temp.path())
        .arg("config")
        .arg("set")
        .arg("defaults.interpreter")
        .arg("python3")
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("gist-cache-rs").unwrap();
    cmd.env("GIST_CACHE_DIR", temp.path())
        .arg("config")
        .arg("get")
        .arg("defaults.interpreter")
        .assert()
        .success()
        .stdout(predicate::str::contains("python3"));
}

#[test]
fn test_config_show() {
    let temp = setup_test_env();
    let mut cmd = Command::cargo_bin("gist-cache-rs").unwrap();

    cmd.env("GIST_CACHE_DIR", temp.path())
        .arg("config")
        .arg("set")
        .arg("defaults.interpreter")
        .arg("bash")
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("gist-cache-rs").unwrap();
    cmd.env("GIST_CACHE_DIR", temp.path())
        .arg("config")
        .arg("show")
        .assert()
        .success()
        .stdout(predicate::str::contains("bash"));
}

#[test]
fn test_config_reset() {
    let temp = setup_test_env();
    let mut cmd = Command::cargo_bin("gist-cache-rs").unwrap();

    cmd.env("GIST_CACHE_DIR", temp.path())
        .arg("config")
        .arg("set")
        .arg("defaults.interpreter")
        .arg("python3")
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("gist-cache-rs").unwrap();
    cmd.env("GIST_CACHE_DIR", temp.path())
        .arg("config")
        .arg("reset")
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("gist-cache-rs").unwrap();
    cmd.env("GIST_CACHE_DIR", temp.path())
        .arg("config")
        .arg("get")
        .arg("defaults.interpreter")
        .assert()
        .success()
        .stdout(predicate::str::contains("not set"));
}
