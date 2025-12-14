// 統合テスト: 実際のスクリプト実行を検証
//
// このテストは外部のインタープリタ（bash, python3, node）に依存します。
// CI環境でこれらがインストールされていることを前提としています。
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

// テスト用のConfigを作成
fn create_test_config() -> (Config, TempDir) {
    let temp_dir = TempDir::new().unwrap();
    let config = Config {
        cache_dir: temp_dir.path().to_path_buf(),
        cache_file: temp_dir.path().join("cache.json"),
        contents_dir: temp_dir.path().join("contents"),
        download_dir: temp_dir.path().join("downloads"),
        config_file: temp_dir.path().join("config.toml"),
        user_config: gist_cache_rs::config::UserConfig::default(),
    };
    fs::create_dir_all(&config.contents_dir).unwrap();
    (config, temp_dir)
}

// fixture のパスを取得
fn get_fixture_path(filename: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(filename)
}

// fixture の内容を読み込み
fn read_fixture(filename: &str) -> String {
    fs::read_to_string(get_fixture_path(filename)).unwrap()
}

// テスト用のGistInfoを作成
fn create_test_gist(id: &str, filename: &str) -> GistInfo {
    GistInfo {
        id: id.to_string(),
        description: Some(format!("Test gist for {}", filename)),
        files: vec![GistFile {
            filename: filename.to_string(),
            language: Some("Shell".to_string()),
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
fn test_execute_bash_script() {
    let (config, _temp_dir) = create_test_config();
    let gist = create_test_gist("test_bash", "hello.sh");

    // fixture の内容をキャッシュに保存
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
        true, // is_shell
        options,
        vec![],
        config,
    );

    // 実行
    let result = runner.run();
    assert!(result.is_ok(), "Bash script should execute successfully");
}

#[test]
#[serial]
#[cfg_attr(not(all(unix, not(target_os = "windows"))), ignore)]
fn test_execute_python_script() {
    let (config, _temp_dir) = create_test_config();
    let gist = create_test_gist("test_python", "hello.py");

    let content = read_fixture("hello.py");
    let content_cache = gist_cache_rs::cache::ContentCache::new(config.contents_dir.clone());
    content_cache.ensure_cache_dir().unwrap();
    content_cache.write(&gist.id, "hello.py", &content).unwrap();

    let options = RunOptions {
        interactive: false,
        preview: false,
        download: false,
        force_file_based: false,
    };

    let runner = ScriptRunner::new(
        gist.clone(),
        "python3".to_string(),
        None,
        false,
        options,
        vec![],
        config,
    );

    let result = runner.run();
    assert!(result.is_ok(), "Python script should execute successfully");
}

#[test]
#[serial]
#[cfg_attr(not(all(unix, not(target_os = "windows"))), ignore)]
fn test_execute_node_script() {
    let (config, _temp_dir) = create_test_config();
    let gist = create_test_gist("test_node", "hello.js");

    let content = read_fixture("hello.js");
    let content_cache = gist_cache_rs::cache::ContentCache::new(config.contents_dir.clone());
    content_cache.ensure_cache_dir().unwrap();
    content_cache.write(&gist.id, "hello.js", &content).unwrap();

    let options = RunOptions {
        interactive: false,
        preview: false,
        download: false,
        force_file_based: false,
    };

    let runner = ScriptRunner::new(
        gist.clone(),
        "node".to_string(),
        None,
        false,
        options,
        vec![],
        config,
    );

    let result = runner.run();
    assert!(result.is_ok(), "Node script should execute successfully");
}

#[test]
#[serial]
#[cfg_attr(not(all(unix, not(target_os = "windows"))), ignore)]
fn test_execute_with_arguments() {
    let (config, _temp_dir) = create_test_config();
    let gist = create_test_gist("test_args", "args_echo.sh");

    let content = read_fixture("args_echo.sh");
    let content_cache = gist_cache_rs::cache::ContentCache::new(config.contents_dir.clone());
    content_cache.ensure_cache_dir().unwrap();
    content_cache
        .write(&gist.id, "args_echo.sh", &content)
        .unwrap();

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
        vec!["arg1".to_string(), "arg2".to_string()],
        config,
    );

    let result = runner.run();
    assert!(
        result.is_ok(),
        "Script with arguments should execute successfully"
    );
}

#[test]
#[serial]
#[cfg_attr(not(all(unix, not(target_os = "windows"))), ignore)]
fn test_execute_failing_script() {
    let (config, _temp_dir) = create_test_config();
    let gist = create_test_gist("test_error", "error_exit.sh");

    let content = read_fixture("error_exit.sh");
    let content_cache = gist_cache_rs::cache::ContentCache::new(config.contents_dir.clone());
    content_cache.ensure_cache_dir().unwrap();
    content_cache
        .write(&gist.id, "error_exit.sh", &content)
        .unwrap();

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
        config,
    );

    let result = runner.run();
    assert!(result.is_err(), "Failing script should return an error");
}

#[test]
#[serial]
#[cfg_attr(not(all(unix, not(target_os = "windows"))), ignore)]
fn test_preview_mode_does_not_execute() {
    let (config, _temp_dir) = create_test_config();
    let gist = create_test_gist("test_preview", "hello.sh");

    let content = read_fixture("hello.sh");
    let content_cache = gist_cache_rs::cache::ContentCache::new(config.contents_dir.clone());
    content_cache.ensure_cache_dir().unwrap();
    content_cache.write(&gist.id, "hello.sh", &content).unwrap();

    let options = RunOptions {
        interactive: false,
        preview: true, // プレビューモード
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
        config,
    );

    let result = runner.run();
    assert!(
        result.is_ok(),
        "Preview mode should succeed without execution"
    );
}

// TypeScript系インタープリタのテスト

#[test]
#[serial]
#[cfg_attr(not(all(unix, not(target_os = "windows"))), ignore)]
fn test_execute_ts_node_script() {
    // ts-nodeがインストールされているか確認
    if std::process::Command::new("ts-node")
        .arg("--version")
        .output()
        .is_err()
    {
        eprintln!("ts-node not found, skipping test");
        return;
    }

    let (config, _temp_dir) = create_test_config();
    let gist = create_test_gist("test_ts_node", "hello.ts");

    let content = read_fixture("hello.ts");
    let content_cache = gist_cache_rs::cache::ContentCache::new(config.contents_dir.clone());
    content_cache.ensure_cache_dir().unwrap();
    content_cache.write(&gist.id, "hello.ts", &content).unwrap();

    let options = RunOptions {
        interactive: false,
        preview: false,
        download: false,
        force_file_based: true, // TypeScriptはファイルベース実行が必須
    };

    let runner = ScriptRunner::new(
        gist.clone(),
        "ts-node".to_string(),
        None,
        false,
        options,
        vec![],
        config,
    );

    let result = runner.run();
    assert!(result.is_ok(), "ts-node script should execute successfully");
}

#[test]
#[serial]
#[cfg_attr(not(all(unix, not(target_os = "windows"))), ignore)]
fn test_execute_deno_script() {
    // denoがインストールされているか確認
    if std::process::Command::new("deno")
        .arg("--version")
        .output()
        .is_err()
    {
        eprintln!("deno not found, skipping test");
        return;
    }

    let (config, _temp_dir) = create_test_config();
    let gist = create_test_gist("test_deno", "hello.ts");

    let content = read_fixture("hello.ts");
    let content_cache = gist_cache_rs::cache::ContentCache::new(config.contents_dir.clone());
    content_cache.ensure_cache_dir().unwrap();
    content_cache.write(&gist.id, "hello.ts", &content).unwrap();

    let options = RunOptions {
        interactive: false,
        preview: false,
        download: false,
        force_file_based: true, // TypeScriptはファイルベース実行が必須
    };

    let runner = ScriptRunner::new(
        gist.clone(),
        "deno".to_string(),
        None,
        false,
        options,
        vec![],
        config,
    );

    let result = runner.run();
    assert!(result.is_ok(), "deno script should execute successfully");
}

#[test]
#[serial]
#[cfg_attr(not(all(unix, not(target_os = "windows"))), ignore)]
fn test_execute_bun_script() {
    // bunがインストールされているか確認
    if std::process::Command::new("bun")
        .arg("--version")
        .output()
        .is_err()
    {
        eprintln!("bun not found, skipping test");
        return;
    }

    let (config, _temp_dir) = create_test_config();
    let gist = create_test_gist("test_bun", "hello.ts");

    let content = read_fixture("hello.ts");
    let content_cache = gist_cache_rs::cache::ContentCache::new(config.contents_dir.clone());
    content_cache.ensure_cache_dir().unwrap();
    content_cache.write(&gist.id, "hello.ts", &content).unwrap();

    let options = RunOptions {
        interactive: false,
        preview: false,
        download: false,
        force_file_based: true, // TypeScriptはファイルベース実行が必須
    };

    let runner = ScriptRunner::new(
        gist.clone(),
        "bun".to_string(),
        None,
        false,
        options,
        vec![],
        config,
    );

    let result = runner.run();
    assert!(result.is_ok(), "bun script should execute successfully");
}

// Windows専用テスト (PowerShell)

#[test]
#[serial]
#[cfg(windows)]
fn test_execute_powershell_script() {
    let (config, _temp_dir) = create_test_config();
    let gist = create_test_gist("test_pwsh", "hello.ps1");

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
        force_file_based: true, // PowerShellはファイルベース実行
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
        "PowerShell script should execute successfully"
    );
}

#[test]
#[serial]
#[cfg(windows)]
fn test_execute_powershell_with_arguments() {
    let (config, _temp_dir) = create_test_config();
    let gist = create_test_gist("test_pwsh_args", "args_echo.ps1");

    let content = read_fixture("args_echo.ps1");
    let content_cache = gist_cache_rs::cache::ContentCache::new(config.contents_dir.clone());
    content_cache.ensure_cache_dir().unwrap();
    content_cache
        .write(&gist.id, "args_echo.ps1", &content)
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
        vec!["arg1".to_string(), "arg2".to_string()],
        config,
    );

    let result = runner.run();
    assert!(
        result.is_ok(),
        "PowerShell script with arguments should execute successfully"
    );
}

#[test]
#[serial]
#[cfg(windows)]
fn test_execute_powershell_failing_script() {
    let (config, _temp_dir) = create_test_config();
    let gist = create_test_gist("test_pwsh_error", "error_exit.ps1");

    let content = read_fixture("error_exit.ps1");
    let content_cache = gist_cache_rs::cache::ContentCache::new(config.contents_dir.clone());
    content_cache.ensure_cache_dir().unwrap();
    content_cache
        .write(&gist.id, "error_exit.ps1", &content)
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
        config,
    );

    let result = runner.run();
    assert!(
        result.is_err(),
        "Failing PowerShell script should return an error"
    );
}

#[test]
#[serial]
#[cfg(windows)]
fn test_execute_powershell_preview_mode() {
    let (config, _temp_dir) = create_test_config();
    let gist = create_test_gist("test_pwsh_preview", "hello.ps1");

    let content = read_fixture("hello.ps1");
    let content_cache = gist_cache_rs::cache::ContentCache::new(config.contents_dir.clone());
    content_cache.ensure_cache_dir().unwrap();
    content_cache
        .write(&gist.id, "hello.ps1", &content)
        .unwrap();

    let options = RunOptions {
        interactive: false,
        preview: true, // プレビューモード
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
        config,
    );

    let result = runner.run();
    assert!(
        result.is_ok(),
        "PowerShell preview mode should succeed without execution"
    );
}

// Ruby, Perl, PHP インタープリタのテスト

#[test]
#[serial]
#[cfg_attr(not(all(unix, not(target_os = "windows"))), ignore)]
fn test_execute_ruby_script() {
    // rubyがインストールされているか確認
    if std::process::Command::new("ruby")
        .arg("--version")
        .output()
        .is_err()
    {
        eprintln!("ruby not found, skipping test");
        return;
    }

    let (config, _temp_dir) = create_test_config();
    let gist = create_test_gist("test_ruby", "hello.rb");

    let content = read_fixture("hello.rb");
    let content_cache = gist_cache_rs::cache::ContentCache::new(config.contents_dir.clone());
    content_cache.ensure_cache_dir().unwrap();
    content_cache.write(&gist.id, "hello.rb", &content).unwrap();

    let options = RunOptions {
        interactive: false,
        preview: false,
        download: false,
        force_file_based: true, // ファイルベース実行
    };

    let runner = ScriptRunner::new(
        gist.clone(),
        "ruby".to_string(),
        None,
        false,
        options,
        vec![],
        config,
    );

    let result = runner.run();
    assert!(result.is_ok(), "ruby script should execute successfully");
}

#[test]
#[serial]
#[cfg_attr(not(all(unix, not(target_os = "windows"))), ignore)]
fn test_execute_perl_script() {
    // perlがインストールされているか確認
    if std::process::Command::new("perl")
        .arg("--version")
        .output()
        .is_err()
    {
        eprintln!("perl not found, skipping test");
        return;
    }

    let (config, _temp_dir) = create_test_config();
    let gist = create_test_gist("test_perl", "hello.pl");

    let content = read_fixture("hello.pl");
    let content_cache = gist_cache_rs::cache::ContentCache::new(config.contents_dir.clone());
    content_cache.ensure_cache_dir().unwrap();
    content_cache.write(&gist.id, "hello.pl", &content).unwrap();

    let options = RunOptions {
        interactive: false,
        preview: false,
        download: false,
        force_file_based: true, // ファイルベース実行
    };

    let runner = ScriptRunner::new(
        gist.clone(),
        "perl".to_string(),
        None,
        false,
        options,
        vec![],
        config,
    );

    let result = runner.run();
    assert!(result.is_ok(), "perl script should execute successfully");
}

#[test]
#[serial]
#[cfg_attr(not(all(unix, not(target_os = "windows"))), ignore)]
fn test_execute_php_script() {
    // phpがインストールされているか確認
    if std::process::Command::new("php")
        .arg("--version")
        .output()
        .is_err()
    {
        eprintln!("php not found, skipping test");
        return;
    }

    let (config, _temp_dir) = create_test_config();
    let gist = create_test_gist("test_php", "hello.php");

    let content = read_fixture("hello.php");
    let content_cache = gist_cache_rs::cache::ContentCache::new(config.contents_dir.clone());
    content_cache.ensure_cache_dir().unwrap();
    content_cache
        .write(&gist.id, "hello.php", &content)
        .unwrap();

    let options = RunOptions {
        interactive: false,
        preview: false,
        download: false,
        force_file_based: true, // phpはファイルベース実行が必須（CLAUDE.mdより）
    };

    let runner = ScriptRunner::new(
        gist.clone(),
        "php".to_string(),
        None,
        false,
        options,
        vec![],
        config,
    );

    let result = runner.run();
    assert!(result.is_ok(), "php script should execute successfully");
}
