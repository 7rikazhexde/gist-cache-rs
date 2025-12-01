use crate::error::{GistCacheError, Result};
use colored::Colorize;
use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Self-update options
#[derive(Debug)]
pub struct UpdateOptions {
    pub from_source: bool,
    pub check: bool,
    pub force: bool,
    pub version: Option<String>,
    pub verbose: bool,
}

/// Self-updater for the application
pub struct Updater {
    options: UpdateOptions,
}

impl Updater {
    /// Create a new updater with the given options
    pub fn new(options: UpdateOptions) -> Self {
        Self { options }
    }

    /// Run the self-update process
    pub fn update(&self) -> Result<()> {
        if self.options.verbose {
            println!("{}", "=== Self-Update ===".cyan().bold());
        }

        // Get current version
        let current_version = env!("CARGO_PKG_VERSION");
        println!("現在のバージョン: {}", current_version.green());

        if self.options.from_source {
            self.update_from_source()
        } else {
            self.update_from_releases()
        }
    }

    /// Update from GitHub Releases
    fn update_from_releases(&self) -> Result<()> {
        if self.options.verbose {
            println!("{}", "GitHub Releasesから更新を確認しています...".cyan());
        }

        // Build updater
        let mut builder = self_update::backends::github::Update::configure();

        // Set binary name with platform-specific extension
        #[cfg(target_os = "windows")]
        let bin_name = "gist-cache-rs.exe";
        #[cfg(not(target_os = "windows"))]
        let bin_name = "gist-cache-rs";

        builder
            .repo_owner("7rikazhexde")
            .repo_name("gist-cache-rs")
            .bin_name(bin_name)
            .show_download_progress(true)
            .current_version(env!("CARGO_PKG_VERSION"));

        // Apply options
        if let Some(ref version) = self.options.version {
            builder.target_version_tag(version);
        } else {
            builder.no_confirm(true);
        }

        // Check mode: just check for updates
        if self.options.check {
            let release = builder.build()?.get_latest_release()?;
            let latest_version = &release.version;

            println!("最新バージョン: {}", latest_version.green());

            if latest_version.as_str() > env!("CARGO_PKG_VERSION") {
                println!(
                    "{}",
                    format!("新しいバージョン {} が利用可能です", latest_version)
                        .yellow()
                        .bold()
                );
                println!("更新するには: {}", "gist-cache-rs self update".cyan());
            } else {
                println!("{}", "最新版を使用しています".green().bold());
            }
            return Ok(());
        }

        // Perform update
        println!("{}", "更新を確認しています...".cyan());

        let status = builder.build()?.update()?;

        match status {
            self_update::Status::UpToDate(version) => {
                println!(
                    "{}",
                    format!("すでに最新版です ({})", version).green().bold()
                );
            }
            self_update::Status::Updated(version) => {
                println!(
                    "{}",
                    format!("更新が完了しました: {}", version).green().bold()
                );
                println!("新しいバージョンで再起動してください。");
            }
        }

        Ok(())
    }

    /// Update from source (git + cargo build + self-replace)
    fn update_from_source(&self) -> Result<()> {
        if self.options.verbose {
            println!("{}", "ソースからビルドして更新します...".cyan());
        }

        // Check if git is available
        if !self.check_command_exists("git") {
            return Err(GistCacheError::SelfUpdate(
                "gitコマンドが見つかりません。gitをインストールしてください。".to_string(),
            ));
        }

        // Check if cargo is available
        if !self.check_command_exists("cargo") {
            return Err(GistCacheError::SelfUpdate(
                "cargoコマンドが見つかりません。Rustをインストールしてください。".to_string(),
            ));
        }

        // Get repository path
        let repo_path = self.get_repository_path()?;

        if self.options.verbose {
            println!("リポジトリパス: {}", repo_path.display().to_string().cyan());
        }

        // Pull latest changes
        println!("{}", "最新の変更を取得しています...".cyan());
        self.run_git_pull(&repo_path)?;

        // Build from source
        println!("{}", "ソースからビルドしています...".cyan());
        self.run_cargo_build(&repo_path)?;

        // Get current executable path
        let current_exe = std::env::current_exe().map_err(|e| {
            GistCacheError::SelfUpdate(format!("実行ファイルのパスを取得できませんでした: {}", e))
        })?;

        // Get path to the newly built binary
        let new_binary =
            repo_path
                .join("target")
                .join("release")
                .join(current_exe.file_name().ok_or_else(|| {
                    GistCacheError::SelfUpdate("実行ファイル名を取得できませんでした".to_string())
                })?);

        if !new_binary.exists() {
            return Err(GistCacheError::SelfUpdate(format!(
                "ビルドされたバイナリが見つかりません: {}",
                new_binary.display()
            )));
        }

        // Replace the current binary with the new one
        println!("{}", "実行ファイルを置き換えています...".cyan());
        self_replace::self_replace(&new_binary).map_err(|e| {
            GistCacheError::SelfUpdate(format!("バイナリの置き換えに失敗しました: {}", e))
        })?;

        println!("{}", "更新が完了しました".green().bold());
        println!("新しいバージョンで再起動してください。");

        Ok(())
    }

    /// Check if a command exists in PATH
    fn check_command_exists(&self, command: &str) -> bool {
        #[cfg(windows)]
        let cmd = "where";
        #[cfg(not(windows))]
        let cmd = "which";

        Command::new(cmd)
            .arg(command)
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }

    /// Get repository path
    fn get_repository_path(&self) -> Result<PathBuf> {
        // 1. Check environment variable
        if let Ok(repo_path) = env::var("GIST_CACHE_REPO") {
            let path = PathBuf::from(repo_path);
            if path.exists() && path.join(".git").exists() {
                return Ok(path);
            }
        }

        // 2. Try to get from cargo metadata (if we're in development)
        if let Ok(output) = Command::new("cargo")
            .args(["metadata", "--format-version", "1", "--no-deps"])
            .output()
        {
            if output.status.success() {
                if let Ok(metadata) = String::from_utf8(output.stdout) {
                    if let Ok(json) = serde_json::from_str::<serde_json::Value>(&metadata) {
                        if let Some(workspace_root) = json["workspace_root"].as_str() {
                            let path = PathBuf::from(workspace_root);
                            if path.join("Cargo.toml").exists() {
                                return Ok(path);
                            }
                        }
                    }
                }
            }
        }

        // 3. Fallback: suggest cloning
        Err(GistCacheError::SelfUpdate(
            "リポジトリが見つかりません。GIST_CACHE_REPO環境変数を設定するか、リポジトリをクローンしてください:\n  git clone https://github.com/7rikazhexde/gist-cache-rs.git".to_string(),
        ))
    }

    /// Run git pull in the repository
    fn run_git_pull(&self, repo_path: &Path) -> Result<()> {
        // First, try to pull with tracking information
        let output = Command::new("git")
            .args(["pull", "--ff-only"])
            .current_dir(repo_path)
            .output()
            .map_err(|e| {
                GistCacheError::SelfUpdate(format!("git pullの実行に失敗しました: {}", e))
            })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);

            // If tracking information is missing, try pulling from origin/main
            if stderr.contains("no tracking information") {
                if self.options.verbose {
                    println!("トラッキング情報がありません。origin/main から取得します...");
                }

                let output = Command::new("git")
                    .args(["pull", "origin", "main", "--ff-only"])
                    .current_dir(repo_path)
                    .output()
                    .map_err(|e| {
                        GistCacheError::SelfUpdate(format!("git pullの実行に失敗しました: {}", e))
                    })?;

                if !output.status.success() {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    return Err(GistCacheError::SelfUpdate(format!(
                        "git pull origin mainに失敗しました: {}",
                        stderr
                    )));
                }

                if self.options.verbose {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    if !stdout.trim().is_empty() {
                        println!("{}", stdout);
                    }
                }

                return Ok(());
            }

            return Err(GistCacheError::SelfUpdate(format!(
                "git pullに失敗しました: {}",
                stderr
            )));
        }

        if self.options.verbose {
            let stdout = String::from_utf8_lossy(&output.stdout);
            if !stdout.trim().is_empty() {
                println!("{}", stdout);
            }
        }

        Ok(())
    }

    /// Run cargo build --release in the repository
    fn run_cargo_build(&self, repo_path: &Path) -> Result<()> {
        let args = vec!["build", "--release"];

        let output = Command::new("cargo")
            .args(&args)
            .current_dir(repo_path)
            .output()
            .map_err(|e| {
                GistCacheError::SelfUpdate(format!("cargo buildの実行に失敗しました: {}", e))
            })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(GistCacheError::SelfUpdate(format!(
                "cargo buildに失敗しました: {}",
                stderr
            )));
        }

        if self.options.verbose {
            let stdout = String::from_utf8_lossy(&output.stdout);
            if !stdout.trim().is_empty() {
                println!("{}", stdout);
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_updater_creation() {
        let options = UpdateOptions {
            from_source: false,
            check: false,
            force: false,
            version: None,
            verbose: false,
        };

        let updater = Updater::new(options);
        assert!(!updater.options.from_source);
        assert!(!updater.options.check);
        assert!(!updater.options.force);
        assert!(updater.options.version.is_none());
        assert!(!updater.options.verbose);
    }

    #[test]
    fn test_updater_with_options() {
        let options = UpdateOptions {
            from_source: true,
            check: true,
            force: true,
            version: Some("0.5.0".to_string()),
            verbose: true,
        };

        let updater = Updater::new(options);
        assert!(updater.options.from_source);
        assert!(updater.options.check);
        assert!(updater.options.force);
        assert_eq!(updater.options.version, Some("0.5.0".to_string()));
        assert!(updater.options.verbose);
    }

    #[test]
    fn test_check_command_exists() {
        let options = UpdateOptions {
            from_source: false,
            check: false,
            force: false,
            version: None,
            verbose: false,
        };

        let updater = Updater::new(options);

        // cargo should exist in test environment
        assert!(updater.check_command_exists("cargo"));

        // non-existent command should return false
        assert!(!updater.check_command_exists("this-command-does-not-exist-12345"));
    }

    #[test]
    fn test_get_repository_path_with_metadata() {
        let options = UpdateOptions {
            from_source: false,
            check: false,
            force: false,
            version: None,
            verbose: false,
        };

        let updater = Updater::new(options);

        // In test environment, cargo metadata should work
        let result = updater.get_repository_path();

        // Should either succeed (if in dev environment) or fail with expected message
        match result {
            Ok(path) => {
                // Should have Cargo.toml
                assert!(path.join("Cargo.toml").exists());
            }
            Err(e) => {
                // Should be the expected error message
                assert!(e.to_string().contains("リポジトリが見つかりません"));
            }
        }
    }

    #[test]
    fn test_get_repository_path_with_env_var() {
        use std::env;

        let options = UpdateOptions {
            from_source: false,
            check: false,
            force: false,
            version: None,
            verbose: false,
        };

        let updater = Updater::new(options);

        // Get current directory (which should be the repo root in tests)
        let current_dir = env::current_dir().unwrap();

        // Set environment variable
        unsafe {
            env::set_var("GIST_CACHE_REPO", current_dir.to_str().unwrap());
        }

        let result = updater.get_repository_path();

        // Clean up
        unsafe {
            env::remove_var("GIST_CACHE_REPO");
        }

        // Should succeed with the current directory
        assert!(result.is_ok());
        let path = result.unwrap();
        assert_eq!(path, current_dir);
    }
}
