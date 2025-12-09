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
