// 統合テスト: ScriptRunnerの詳細な動作を検証
//
// このテストは実際のファイルシステムとプロセス実行に依存します。
//
// 注意: これらのテストは並行実行時に競合状態が発生する可能性があるため、
// #[serial]属性を使用して順次実行されます。

use chrono::Utc;
use gist_cache_rs::cache::types::{GistFile, GistInfo};
use gist_cache_rs::config::Config;
use gist_cache_rs::execution::runner::{RunOptions, ScriptRunner};
use serial_test::serial;
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

fn create_test_config() -> (Config, TempDir) {
    let temp_dir = TempDir::new().unwrap();
    let config = Config {
        cache_dir: temp_dir.path().to_path_buf(),
        cache_file: temp_dir.path().join("cache.json"),
        contents_dir: temp_dir.path().join("contents"),
        download_dir: temp_dir.path().join("downloads"),
    };
    fs::create_dir_all(&config.contents_dir).unwrap();
    fs::create_dir_all(&config.download_dir).unwrap();
    (config, temp_dir)
}

fn get_fixture_path(filename: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(filename)
}

fn read_fixture(filename: &str) -> String {
    fs::read_to_string(get_fixture_path(filename)).unwrap()
}

fn create_test_gist(id: &str, filename: &str, lang: Option<&str>) -> GistInfo {
    GistInfo {
        id: id.to_string(),
        description: Some(format!("Test gist for {}", filename)),
        files: vec![GistFile {
            filename: filename.to_string(),
            language: lang.map(|s| s.to_string()),
            size: 100,
        }],
        updated_at: Utc::now(),
        public: true,
        html_url: format!("https://gist.github.com/{}", id),
    }
}

#[test]
#[serial]
#[cfg_attr(not(all(unix, not(target_os = "windows"))), ignore)]
fn test_download_mode_creates_file() {
    let (config, _temp_dir) = create_test_config();
    let gist = create_test_gist("test_download", "hello.sh", Some("Shell"));

    let content = read_fixture("hello.sh");
    let content_cache = gist_cache_rs::cache::ContentCache::new(config.contents_dir.clone());
    content_cache.ensure_cache_dir().unwrap();
    content_cache.write(&gist.id, "hello.sh", &content).unwrap();

    let options = RunOptions {
        interactive: false,
        preview: false,
        download: true, // ダウンロードモード
        force_file_based: false,
    };

    let runner = ScriptRunner::new(
        gist.clone(),
        "bash".to_string(),
        None,
        true,
        options,
        vec![],
        config.clone(),
    );

    let result = runner.run();
    assert!(result.is_ok(), "Download mode should succeed");

    // ダウンロードファイルが作成されたか確認
    let download_path = config.download_dir.join("hello.sh");
    assert!(
        download_path.exists(),
        "Downloaded file should exist in download directory"
    );

    let downloaded_content = fs::read_to_string(download_path).unwrap();
    assert_eq!(
        downloaded_content, content,
        "Downloaded content should match"
    );
}

#[test]
#[serial]
#[cfg_attr(not(all(unix, not(target_os = "windows"))), ignore)]
fn test_preview_with_download_mode() {
    let (config, _temp_dir) = create_test_config();
    let gist = create_test_gist("test_preview_dl", "hello.sh", Some("Shell"));

    let content = read_fixture("hello.sh");
    let content_cache = gist_cache_rs::cache::ContentCache::new(config.contents_dir.clone());
    content_cache.ensure_cache_dir().unwrap();
    content_cache.write(&gist.id, "hello.sh", &content).unwrap();

    let options = RunOptions {
        interactive: false,
        preview: true,  // プレビュー
        download: true, // ダウンロード
        force_file_based: false,
    };

    let runner = ScriptRunner::new(
        gist.clone(),
        "bash".to_string(),
        None,
        true,
        options,
        vec![],
        config.clone(),
    );

    let result = runner.run();
    assert!(result.is_ok(), "Preview + Download should succeed");

    // ダウンロードファイルが作成されたか確認
    let download_path = config.download_dir.join("hello.sh");
    assert!(
        download_path.exists(),
        "Downloaded file should exist even in preview mode"
    );
}

#[test]
#[serial]
#[cfg_attr(not(all(unix, not(target_os = "windows"))), ignore)]
fn test_cache_creation_after_execution() {
    let (config, _temp_dir) = create_test_config();
    let gist = create_test_gist("test_cache_creation", "hello.sh", Some("Shell"));

    let content = read_fixture("hello.sh");
    let content_cache = gist_cache_rs::cache::ContentCache::new(config.contents_dir.clone());
    content_cache.ensure_cache_dir().unwrap();

    // 初回実行前はキャッシュが存在しない
    assert!(!content_cache.exists(&gist.id, "hello.sh"));

    // コンテンツを手動で設定（GitHub APIから取得したと仮定）
    content_cache.write(&gist.id, "hello.sh", &content).unwrap();

    let options = RunOptions {
        interactive: false,
        preview: false,
        download: false,
        force_file_based: false,
    };

    let runner = ScriptRunner::new(
        gist.clone(),
        "bash".to_string(),
        None,
        true,
        options,
        vec![],
        config.clone(),
    );

    let result = runner.run();
    assert!(result.is_ok(), "Execution should succeed");

    // 実行後にキャッシュが存在する
    assert!(
        content_cache.exists(&gist.id, "hello.sh"),
        "Cache should exist after execution"
    );
}

#[test]
#[serial]
#[cfg_attr(not(all(unix, not(target_os = "windows"))), ignore)]
fn test_multiple_files_gist() {
    let (config, _temp_dir) = create_test_config();

    let mut gist = create_test_gist("test_multi", "hello.sh", Some("Shell"));
    gist.files.push(GistFile {
        filename: "hello.py".to_string(),
        language: Some("Python".to_string()),
        size: 100,
    });

    let content = read_fixture("hello.sh");
    let content_cache = gist_cache_rs::cache::ContentCache::new(config.contents_dir.clone());
    content_cache.ensure_cache_dir().unwrap();
    content_cache.write(&gist.id, "hello.sh", &content).unwrap();

    let options = RunOptions {
        interactive: false,
        preview: false,
        download: false,
        force_file_based: false,
    };

    // bash インタープリタを指定した場合、.sh ファイルが選択されるべき
    let runner = ScriptRunner::new(
        gist.clone(),
        "bash".to_string(),
        None,
        true,
        options,
        vec![],
        config,
    );

    let result = runner.run();
    assert!(
        result.is_ok(),
        "Should select correct file from multiple files"
    );
}

#[test]
#[serial]
#[cfg_attr(not(all(unix, not(target_os = "windows"))), ignore)]
fn test_force_file_based_execution() {
    let (config, _temp_dir) = create_test_config();
    let gist = create_test_gist("test_file_based", "hello.sh", Some("Shell"));

    let content = read_fixture("hello.sh");
    let content_cache = gist_cache_rs::cache::ContentCache::new(config.contents_dir.clone());
    content_cache.ensure_cache_dir().unwrap();
    content_cache.write(&gist.id, "hello.sh", &content).unwrap();

    let options = RunOptions {
        interactive: false,
        preview: false,
        download: false,
        force_file_based: true, // ファイルベース実行を強制
    };

    let runner = ScriptRunner::new(
        gist.clone(),
        "bash".to_string(),
        None,
        true,
        options,
        vec![],
        config,
    );

    let result = runner.run();
    assert!(result.is_ok(), "Force file-based execution should succeed");
}

#[test]
#[serial]
#[cfg_attr(not(all(unix, not(target_os = "windows"))), ignore)]
fn test_script_with_empty_arguments() {
    let (config, _temp_dir) = create_test_config();
    let gist = create_test_gist("test_no_args", "hello.sh", Some("Shell"));

    let content = read_fixture("hello.sh");
    let content_cache = gist_cache_rs::cache::ContentCache::new(config.contents_dir.clone());
    content_cache.ensure_cache_dir().unwrap();
    content_cache.write(&gist.id, "hello.sh", &content).unwrap();

    let options = RunOptions {
        interactive: false,
        preview: false,
        download: false,
        force_file_based: false,
    };

    let runner = ScriptRunner::new(
        gist.clone(),
        "bash".to_string(),
        None,
        true,
        options,
        vec![], // 空の引数リスト
        config,
    );

    let result = runner.run();
    assert!(result.is_ok(), "Script with no arguments should succeed");
}

// Windows専用テスト (PowerShell)

#[test]
#[serial]
#[cfg(windows)]
fn test_powershell_download_mode() {
    let (config, _temp_dir) = create_test_config();
    let gist = create_test_gist("test_pwsh_download", "hello.ps1", Some("PowerShell"));

    let content = read_fixture("hello.ps1");
    let content_cache = gist_cache_rs::cache::ContentCache::new(config.contents_dir.clone());
    content_cache.ensure_cache_dir().unwrap();
    content_cache
        .write(&gist.id, "hello.ps1", &content)
        .unwrap();

    let options = RunOptions {
        interactive: false,
        preview: false,
        download: true, // ダウンロードモード
        force_file_based: true,
    };

    let runner = ScriptRunner::new(
        gist.clone(),
        "pwsh".to_string(),
        None,
        false,
        options,
        vec![],
        config.clone(),
    );

    let result = runner.run();
    assert!(result.is_ok(), "PowerShell download mode should succeed");

    // ダウンロードファイルが作成されたか確認
    let download_path = config.download_dir.join("hello.ps1");
    assert!(
        download_path.exists(),
        "Downloaded file should exist in download directory"
    );

    let downloaded_content = fs::read_to_string(download_path).unwrap();
    assert_eq!(
        downloaded_content, content,
        "Downloaded content should match"
    );
}

#[test]
#[serial]
#[cfg(windows)]
fn test_powershell_cache_creation() {
    let (config, _temp_dir) = create_test_config();
    let gist = create_test_gist("test_pwsh_cache", "hello.ps1", Some("PowerShell"));

    let content = read_fixture("hello.ps1");
    let content_cache = gist_cache_rs::cache::ContentCache::new(config.contents_dir.clone());
    content_cache.ensure_cache_dir().unwrap();

    // 初回実行前はキャッシュが存在しない
    assert!(!content_cache.exists(&gist.id, "hello.ps1"));

    // コンテンツを手動で設定（GitHub APIから取得したと仮定）
    content_cache
        .write(&gist.id, "hello.ps1", &content)
        .unwrap();

    let options = RunOptions {
        interactive: false,
        preview: false,
        download: false,
        force_file_based: true,
    };

    let runner = ScriptRunner::new(
        gist.clone(),
        "pwsh".to_string(),
        None,
        false,
        options,
        vec![],
        config.clone(),
    );

    let result = runner.run();
    assert!(result.is_ok(), "PowerShell execution should succeed");

    // 実行後にキャッシュが存在する
    assert!(
        content_cache.exists(&gist.id, "hello.ps1"),
        "Cache should exist after execution"
    );
}

#[test]
#[serial]
#[cfg(windows)]
fn test_powershell_force_file_based() {
    let (config, _temp_dir) = create_test_config();
    let gist = create_test_gist("test_pwsh_force", "hello.ps1", Some("PowerShell"));

    let content = read_fixture("hello.ps1");
    let content_cache = gist_cache_rs::cache::ContentCache::new(config.contents_dir.clone());
    content_cache.ensure_cache_dir().unwrap();
    content_cache
        .write(&gist.id, "hello.ps1", &content)
        .unwrap();

    let options = RunOptions {
        interactive: false,
        preview: false,
        download: false,
        force_file_based: true, // ファイルベース実行を強制
    };

    let runner = ScriptRunner::new(
        gist.clone(),
        "pwsh".to_string(),
        None,
        false,
        options,
        vec![],
        config,
    );

    let result = runner.run();
    assert!(
        result.is_ok(),
        "PowerShell force file-based execution should succeed"
    );
}

#[test]
#[serial]
#[cfg(windows)]
fn test_powershell_preview_with_download() {
    let (config, _temp_dir) = create_test_config();
    let gist = create_test_gist("test_pwsh_preview_dl", "hello.ps1", Some("PowerShell"));

    let content = read_fixture("hello.ps1");
    let content_cache = gist_cache_rs::cache::ContentCache::new(config.contents_dir.clone());
    content_cache.ensure_cache_dir().unwrap();
    content_cache
        .write(&gist.id, "hello.ps1", &content)
        .unwrap();

    let options = RunOptions {
        interactive: false,
        preview: true,  // プレビュー
        download: true, // ダウンロード
        force_file_based: true,
    };

    let runner = ScriptRunner::new(
        gist.clone(),
        "pwsh".to_string(),
        None,
        false,
        options,
        vec![],
        config.clone(),
    );

    let result = runner.run();
    assert!(
        result.is_ok(),
        "PowerShell preview + download should succeed"
    );

    // ダウンロードファイルが作成されたか確認
    let download_path = config.download_dir.join("hello.ps1");
    assert!(
        download_path.exists(),
        "Downloaded file should exist even in preview mode"
    );
}

#[test]
#[serial]
#[cfg(windows)]
fn test_powershell_multiple_files_gist() {
    let (config, _temp_dir) = create_test_config();

    let mut gist = create_test_gist("test_pwsh_multi", "hello.ps1", Some("PowerShell"));
    gist.files.push(GistFile {
        filename: "hello.py".to_string(),
        language: Some("Python".to_string()),
        size: 100,
    });

    let content = read_fixture("hello.ps1");
    let content_cache = gist_cache_rs::cache::ContentCache::new(config.contents_dir.clone());
    content_cache.ensure_cache_dir().unwrap();
    content_cache
        .write(&gist.id, "hello.ps1", &content)
        .unwrap();

    let options = RunOptions {
        interactive: false,
        preview: false,
        download: false,
        force_file_based: true,
    };

    // pwsh インタープリタを指定した場合、.ps1 ファイルが選択されるべき
    let runner = ScriptRunner::new(
        gist.clone(),
        "pwsh".to_string(),
        None,
        false,
        options,
        vec![],
        config,
    );

    let result = runner.run();
    assert!(
        result.is_ok(),
        "PowerShell should select correct file from multiple files"
    );
}

#[test]
#[serial]
#[cfg(windows)]
fn test_powershell_with_empty_arguments() {
    let (config, _temp_dir) = create_test_config();
    let gist = create_test_gist("test_pwsh_no_args", "hello.ps1", Some("PowerShell"));

    let content = read_fixture("hello.ps1");
    let content_cache = gist_cache_rs::cache::ContentCache::new(config.contents_dir.clone());
    content_cache.ensure_cache_dir().unwrap();
    content_cache
        .write(&gist.id, "hello.ps1", &content)
        .unwrap();

    let options = RunOptions {
        interactive: false,
        preview: false,
        download: false,
        force_file_based: true,
    };

    let runner = ScriptRunner::new(
        gist.clone(),
        "pwsh".to_string(),
        None,
        false,
        options,
        vec![], // 空の引数リスト
        config,
    );

    let result = runner.run();
    assert!(
        result.is_ok(),
        "PowerShell script with no arguments should succeed"
    );
}
