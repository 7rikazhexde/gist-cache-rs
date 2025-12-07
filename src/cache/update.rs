use crate::cache::ContentCache;
use crate::cache::types::{CacheMetadata, GistCache, GistInfo};
use crate::config::Config;
use crate::error::Result;
use crate::github::{GitHubApi, GitHubClient};
use chrono::Utc;
use colored::Colorize;
use std::collections::HashMap;
use std::fs;

pub struct CacheUpdater<C: GitHubClient = GitHubApi> {
    config: Config,
    verbose: bool,
    client: C,
}

impl CacheUpdater<GitHubApi> {
    pub fn new(config: Config, verbose: bool) -> Self {
        Self {
            config,
            verbose,
            client: GitHubApi::new(),
        }
    }
}

impl<C: GitHubClient> CacheUpdater<C> {
    pub fn new_with_client(config: Config, verbose: bool, client: C) -> Self {
        Self {
            config,
            verbose,
            client,
        }
    }

    pub fn update(&self, force: bool) -> Result<()> {
        println!("{}", "Updating Gist cache...".cyan());

        if self.verbose {
            if force {
                println!("{}", "Mode: Force full update".yellow());
            } else {
                println!("{}", "Mode: Differential update".yellow());
            }
        }

        // Ensure cache directory exists
        self.config.ensure_cache_dir()?;

        // Create ContentCache instance
        let content_cache = ContentCache::new(self.config.contents_dir.clone());
        content_cache.ensure_cache_dir()?;

        // Check authentication
        self.client.check_auth()?;

        // Check rate limit
        let rate_limit = self.client.check_rate_limit()?;
        if rate_limit < 100 {
            println!(
                "{}",
                format!("Warning: Rate limit remaining is low at {}", rate_limit).yellow()
            );
        }
        if self.verbose {
            println!(
                "{}",
                format!("Rate limit remaining: {}", rate_limit).green()
            );
        }

        // Load existing cache if available
        let (github_user, last_updated, old_gists) = if self.config.cache_exists() && !force {
            let cache = self.load_cache()?;
            if self.verbose {
                println!("{}", "Detected existing cache".green());
                println!(
                    "{}",
                    format!("GitHub user (cache reused): {}", cache.metadata.github_user).green()
                );
                println!(
                    "{}",
                    format!(
                        "Last updated: {}",
                        cache.metadata.last_updated.format("%Y-%m-%dT%H:%M:%SZ")
                    )
                    .green()
                );
            }
            (
                cache.metadata.github_user,
                Some(cache.metadata.last_updated),
                Some(cache.gists),
            )
        } else {
            (self.client.get_user()?, None, None)
        };

        if self.verbose && last_updated.is_none() {
            println!("{}", format!("GitHub user: {}", github_user).green());
        }

        // Fetch gists from GitHub
        if self.verbose {
            println!("{}", "Fetching Gist information from GitHub API...".cyan());
        }

        let since = if force { None } else { last_updated };
        let fetched_gists = self.client.fetch_gists(since)?;
        let fetched_count = fetched_gists.len();

        if self.verbose {
            println!("{}", format!("Fetched Gists: {}", fetched_count).green());
        }

        // Compare metadata and identify cache to be deleted
        let mut deleted_cache_count = 0;
        if let Some(ref old) = old_gists {
            // Convert old metadata to Map
            let old_map: HashMap<String, &GistInfo> =
                old.iter().map(|g| (g.id.clone(), g)).collect();

            // Detect Gists with changed updated_at from newly fetched ones
            for new_gist in &fetched_gists {
                if let Some(old_gist) = old_map.get(&new_gist.id) {
                    // If updated_at is different, the Gist has been updated
                    if old_gist.updated_at != new_gist.updated_at {
                        // Delete cache
                        if self.verbose {
                            println!(
                                "{}",
                                format!(
                                    "Detected Gist update: {} ({})",
                                    new_gist.id,
                                    new_gist
                                        .description
                                        .as_ref()
                                        .unwrap_or(&"No description".to_string())
                                )
                                .yellow()
                            );
                        }

                        // Self-healing principle: Log and continue even if error occurs
                        match content_cache.delete_gist(&new_gist.id) {
                            Ok(deleted) => {
                                if deleted {
                                    // Count only when actually deleted
                                    deleted_cache_count += 1;
                                    if self.verbose {
                                        println!(
                                            "{}",
                                            format!("  → Deleted cache: {}", new_gist.id).green()
                                        );
                                    }
                                } else if self.verbose {
                                    // If it didn't exist (display only in verbose mode)
                                    println!(
                                        "{}",
                                        format!("  → Cache did not exist: {}", new_gist.id).cyan()
                                    );
                                }
                            }
                            Err(e) => {
                                eprintln!(
                                    "{}",
                                    format!(
                                        "  Warning: Failed to delete cache: {} - {}",
                                        new_gist.id, e
                                    )
                                    .yellow()
                                );
                            }
                        }
                    }
                }
            }
        }

        // Merge with existing cache if doing differential update
        let final_gists = if let Some(mut old) = old_gists {
            if fetched_count == 0 {
                println!("{}", "No updates".green());
                old
            } else {
                // Merge by ID, keeping the latest version
                let mut gist_map: HashMap<String, GistInfo> =
                    old.drain(..).map(|g| (g.id.clone(), g)).collect();

                let old_count = gist_map.len();

                for gh_gist in fetched_gists {
                    let gist_info = GistInfo::from(gh_gist);
                    gist_map.insert(gist_info.id.clone(), gist_info);
                }

                let mut merged: Vec<GistInfo> = gist_map.into_values().collect();

                // Sort by updated_at descending (most recent first)
                merged.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));

                let total_count = merged.len();
                let new = total_count - old_count;

                if self.verbose {
                    println!(
                        "{}",
                        format!(
                            "Differential merge completed: Existing {} + Diff {} → Total {}",
                            old_count, fetched_count, total_count
                        )
                        .green()
                    );
                }

                println!("{}", format!("Updated: {} items", fetched_count).green());
                if new > 0 && self.verbose {
                    println!("{}", format!("New Gists: {} items", new).green());
                }

                // Report cache deletion
                if deleted_cache_count > 0 {
                    println!(
                        "{}",
                        format!("Cache deleted: {} items", deleted_cache_count).yellow()
                    );
                }

                merged
            }
        } else {
            // Force update or first time
            let mut gists: Vec<GistInfo> = fetched_gists.into_iter().map(GistInfo::from).collect();

            // Sort by updated_at descending
            gists.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));

            let count = gists.len();
            println!("{}", format!("New/Updated: {} items", count).green());
            gists
        };

        // Create cache data
        let cache = GistCache {
            metadata: CacheMetadata {
                last_updated: Utc::now(),
                total_count: final_gists.len(),
                github_user,
            },
            gists: final_gists,
        };

        // Save to file
        self.save_cache(&cache)?;

        println!("{}", "Cache update completed".green().bold());
        println!(
            "{}",
            format!("Total Gists: {}", cache.metadata.total_count)
                .cyan()
                .bold()
        );

        Ok(())
    }

    fn load_cache(&self) -> Result<GistCache> {
        let content = fs::read_to_string(&self.config.cache_file)?;
        let cache: GistCache = serde_json::from_str(&content)?;
        Ok(cache)
    }

    fn save_cache(&self, cache: &GistCache) -> Result<()> {
        let json = serde_json::to_string_pretty(cache)?;
        fs::write(&self.config.cache_file, json)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cache::types::GitHubFile;
    use crate::github::MockGitHubClient;
    use chrono::Utc;
    use std::collections::HashMap;
    use std::fs;
    use tempfile::TempDir;

    fn create_test_config() -> Config {
        let temp_dir = TempDir::new().unwrap();
        let cache_dir = temp_dir.path().join("cache");
        fs::create_dir_all(&cache_dir).unwrap();

        Config {
            cache_dir: cache_dir.clone(),
            cache_file: cache_dir.join("cache.json"),
            contents_dir: cache_dir.join("contents"),
            download_dir: temp_dir.path().join("downloads"),
        }
    }

    fn create_test_cache() -> GistCache {
        GistCache {
            metadata: CacheMetadata {
                last_updated: Utc::now(),
                total_count: 1,
                github_user: "testuser".to_string(),
            },
            gists: vec![GistInfo {
                id: "test123".to_string(),
                description: Some("Test gist".to_string()),
                files: vec![crate::cache::types::GistFile {
                    filename: "test.sh".to_string(),
                    language: Some("Shell".to_string()),
                    size: 100,
                }],
                updated_at: Utc::now(),
                public: true,
                html_url: "https://gist.github.com/test123".to_string(),
            }],
        }
    }

    #[test]
    fn test_updater_new() {
        let config = create_test_config();
        let updater = CacheUpdater::new(config.clone(), false);
        assert!(!updater.verbose);

        let updater_verbose = CacheUpdater::new(config, true);
        assert!(updater_verbose.verbose);
    }

    #[test]
    fn test_save_and_load_cache() {
        let config = create_test_config();
        config.ensure_cache_dir().unwrap();
        let updater = CacheUpdater::new(config.clone(), false);

        let cache = create_test_cache();

        // Save cache
        updater.save_cache(&cache).unwrap();
        assert!(config.cache_file.exists());

        // Load cache
        let loaded_cache = updater.load_cache().unwrap();
        assert_eq!(loaded_cache.metadata.github_user, "testuser");
        assert_eq!(loaded_cache.gists.len(), 1);
        assert_eq!(loaded_cache.gists[0].id, "test123");
    }

    #[test]
    fn test_load_cache_missing_file() {
        let config = create_test_config();
        let updater = CacheUpdater::new(config, false);

        // Try to load non-existent cache
        let result = updater.load_cache();
        assert!(result.is_err());
    }

    #[test]
    fn test_save_cache_invalid_json() {
        let config = create_test_config();
        config.ensure_cache_dir().unwrap();

        // Write invalid JSON
        fs::write(&config.cache_file, "invalid json").unwrap();

        let updater = CacheUpdater::new(config, false);

        // Try to load invalid cache
        let result = updater.load_cache();
        assert!(result.is_err());
    }

    #[test]
    fn test_cache_updater_with_verbose() {
        let config = create_test_config();
        let updater = CacheUpdater::new(config, true);
        assert!(updater.verbose);
    }

    #[test]
    fn test_cache_metadata() {
        let cache = create_test_cache();
        assert_eq!(cache.metadata.github_user, "testuser");
        assert_eq!(cache.metadata.total_count, 1);
    }

    // モックを使用したupdate()メソッドのテスト

    #[test]
    fn test_update_force_with_mock() {
        let config = create_test_config();
        config.ensure_cache_dir().unwrap();

        let mut mock = MockGitHubClient::new();

        // check_auth()のモック設定
        mock.expect_check_auth().times(1).returning(|| Ok(()));

        // check_rate_limit()のモック設定
        mock.expect_check_rate_limit()
            .times(1)
            .returning(|| Ok(5000));

        // get_user()のモック設定
        mock.expect_get_user()
            .times(1)
            .returning(|| Ok("mockuser".to_string()));

        // fetch_gists()のモック設定（強制更新なのでsinceはNone）
        mock.expect_fetch_gists()
            .times(1)
            .withf(|since| since.is_none())
            .returning(|_| {
                Ok(vec![crate::cache::types::GitHubGist {
                    id: "mock123".to_string(),
                    description: Some("Mock gist".to_string()),
                    files: HashMap::from([(
                        "test.sh".to_string(),
                        GitHubFile {
                            filename: "test.sh".to_string(),
                            language: Some("Shell".to_string()),
                            size: 50,
                        },
                    )]),
                    updated_at: Utc::now(),
                    public: true,
                    html_url: "https://gist.github.com/mock123".to_string(),
                }])
            });

        let updater = CacheUpdater::new_with_client(config.clone(), false, mock);
        let result = updater.update(true); // force = true

        assert!(result.is_ok());
        assert!(config.cache_file.exists());

        // キャッシュファイルの内容を確認
        let loaded = updater.load_cache().unwrap();
        assert_eq!(loaded.metadata.github_user, "mockuser");
        assert_eq!(loaded.gists.len(), 1);
        assert_eq!(loaded.gists[0].id, "mock123");
    }

    #[test]
    fn test_update_differential_with_mock() {
        let config = create_test_config();
        config.ensure_cache_dir().unwrap();

        // 既存のキャッシュを作成
        let existing_cache = create_test_cache();
        let updater_temp = CacheUpdater::new(config.clone(), false);
        updater_temp.save_cache(&existing_cache).unwrap();

        let mut mock = MockGitHubClient::new();

        mock.expect_check_auth().times(1).returning(|| Ok(()));
        mock.expect_check_rate_limit()
            .times(1)
            .returning(|| Ok(4500));

        // 差分更新なので、fetch_gists()はsinceパラメータを受け取る
        mock.expect_fetch_gists()
            .times(1)
            .withf(|since| since.is_some())
            .returning(|_| {
                // 新しいGistを1件返す
                Ok(vec![crate::cache::types::GitHubGist {
                    id: "new456".to_string(),
                    description: Some("New gist".to_string()),
                    files: HashMap::from([(
                        "new.py".to_string(),
                        GitHubFile {
                            filename: "new.py".to_string(),
                            language: Some("Python".to_string()),
                            size: 200,
                        },
                    )]),
                    updated_at: Utc::now(),
                    public: true,
                    html_url: "https://gist.github.com/new456".to_string(),
                }])
            });

        let updater = CacheUpdater::new_with_client(config.clone(), false, mock);
        let result = updater.update(false); // force = false

        assert!(result.is_ok());

        // マージ後のキャッシュを確認
        let loaded = updater.load_cache().unwrap();
        assert_eq!(loaded.gists.len(), 2); // 既存1 + 新規1
    }

    #[test]
    fn test_update_with_no_changes() {
        let config = create_test_config();
        config.ensure_cache_dir().unwrap();

        // 既存のキャッシュを作成
        let existing_cache = create_test_cache();
        let updater_temp = CacheUpdater::new(config.clone(), false);
        updater_temp.save_cache(&existing_cache).unwrap();

        let mut mock = MockGitHubClient::new();

        mock.expect_check_auth().times(1).returning(|| Ok(()));
        mock.expect_check_rate_limit()
            .times(1)
            .returning(|| Ok(3000));

        // fetch_gists()は空の配列を返す（更新なし）
        mock.expect_fetch_gists().times(1).returning(|_| Ok(vec![]));

        let updater = CacheUpdater::new_with_client(config.clone(), false, mock);
        let result = updater.update(false);

        assert!(result.is_ok());

        // キャッシュは変更されていないことを確認
        let loaded = updater.load_cache().unwrap();
        assert_eq!(loaded.gists.len(), 1); // 既存のまま
        assert_eq!(loaded.gists[0].id, "test123");
    }

    #[test]
    fn test_update_with_rate_limit_warning() {
        let config = create_test_config();
        config.ensure_cache_dir().unwrap();

        let mut mock = MockGitHubClient::new();

        mock.expect_check_auth().times(1).returning(|| Ok(()));

        // レート制限が低い値を返す
        mock.expect_check_rate_limit().times(1).returning(|| Ok(50));

        mock.expect_get_user()
            .times(1)
            .returning(|| Ok("testuser".to_string()));

        mock.expect_fetch_gists().times(1).returning(|_| Ok(vec![]));

        let updater = CacheUpdater::new_with_client(config, false, mock);
        let result = updater.update(true);

        // レート制限警告が出ても、更新自体は成功する
        assert!(result.is_ok());
    }

    #[test]
    fn test_update_auth_failure() {
        let config = create_test_config();
        config.ensure_cache_dir().unwrap();

        let mut mock = MockGitHubClient::new();

        // 認証失敗をシミュレート
        mock.expect_check_auth()
            .times(1)
            .returning(|| Err(crate::error::GistCacheError::NotAuthenticated));

        let updater = CacheUpdater::new_with_client(config, false, mock);
        let result = updater.update(false);

        // 認証エラーが返る
        assert!(result.is_err());
    }

    #[test]
    fn test_update_with_gist_modification_deletes_cache() {
        let config = create_test_config();
        config.ensure_cache_dir().unwrap();

        // 既存のキャッシュを作成
        let old_time = Utc::now() - chrono::Duration::hours(1);
        let existing_cache = GistCache {
            metadata: CacheMetadata {
                last_updated: old_time,
                total_count: 1,
                github_user: "testuser".to_string(),
            },
            gists: vec![GistInfo {
                id: "update123".to_string(),
                description: Some("Old description".to_string()),
                files: vec![crate::cache::types::GistFile {
                    filename: "old.sh".to_string(),
                    language: Some("Shell".to_string()),
                    size: 100,
                }],
                updated_at: old_time,
                public: true,
                html_url: "https://gist.github.com/update123".to_string(),
            }],
        };

        let updater_temp = CacheUpdater::new(config.clone(), false);
        updater_temp.save_cache(&existing_cache).unwrap();

        // コンテンツキャッシュを作成
        let content_cache = ContentCache::new(config.contents_dir.clone());
        content_cache.ensure_cache_dir().unwrap();
        content_cache
            .write("update123", "old.sh", "echo old")
            .unwrap();

        // キャッシュが存在することを確認
        assert!(content_cache.exists("update123", "old.sh"));

        let mut mock = MockGitHubClient::new();

        mock.expect_check_auth().times(1).returning(|| Ok(()));
        mock.expect_check_rate_limit()
            .times(1)
            .returning(|| Ok(4000));

        // 更新されたGistを返す（updated_atが新しい）
        let new_time = Utc::now();
        mock.expect_fetch_gists().times(1).returning(move |_| {
            Ok(vec![crate::cache::types::GitHubGist {
                id: "update123".to_string(),
                description: Some("Updated description".to_string()),
                files: HashMap::from([(
                    "new.sh".to_string(),
                    GitHubFile {
                        filename: "new.sh".to_string(),
                        language: Some("Shell".to_string()),
                        size: 120,
                    },
                )]),
                updated_at: new_time,
                public: true,
                html_url: "https://gist.github.com/update123".to_string(),
            }])
        });

        let updater = CacheUpdater::new_with_client(config.clone(), false, mock);
        let result = updater.update(false);

        assert!(result.is_ok());

        // キャッシュが削除されたことを確認
        assert!(!content_cache.exists("update123", "old.sh"));

        // 更新後のメタデータを確認
        let loaded = updater.load_cache().unwrap();
        assert_eq!(loaded.gists.len(), 1);
        assert_eq!(
            loaded.gists[0].description.as_ref().unwrap(),
            "Updated description"
        );
    }

    #[test]
    fn test_update_verbose_mode() {
        let config = create_test_config();
        config.ensure_cache_dir().unwrap();

        let mut mock = MockGitHubClient::new();

        mock.expect_check_auth().times(1).returning(|| Ok(()));
        mock.expect_check_rate_limit()
            .times(1)
            .returning(|| Ok(5000));
        mock.expect_get_user()
            .times(1)
            .returning(|| Ok("verboseuser".to_string()));
        mock.expect_fetch_gists().times(1).returning(|_| Ok(vec![]));

        // verboseモードで実行
        let updater = CacheUpdater::new_with_client(config, true, mock);
        let result = updater.update(true);

        assert!(result.is_ok());
    }

    #[test]
    fn test_update_differential_with_existing_cache_verbose() {
        let config = create_test_config();
        config.ensure_cache_dir().unwrap();

        // 既存のキャッシュを作成
        let existing_cache = create_test_cache();
        let updater_temp = CacheUpdater::new(config.clone(), false);
        updater_temp.save_cache(&existing_cache).unwrap();

        let mut mock = MockGitHubClient::new();

        mock.expect_check_auth().times(1).returning(|| Ok(()));
        mock.expect_check_rate_limit()
            .times(1)
            .returning(|| Ok(3500));
        mock.expect_fetch_gists().times(1).returning(|_| Ok(vec![]));

        // verboseモードで差分更新
        let updater = CacheUpdater::new_with_client(config, true, mock);
        let result = updater.update(false);

        assert!(result.is_ok());
    }

    #[test]
    fn test_update_with_low_rate_limit_verbose() {
        let config = create_test_config();
        config.ensure_cache_dir().unwrap();

        let mut mock = MockGitHubClient::new();

        mock.expect_check_auth().times(1).returning(|| Ok(()));
        mock.expect_check_rate_limit().times(1).returning(|| Ok(80)); // Low rate limit
        mock.expect_get_user()
            .times(1)
            .returning(|| Ok("testuser".to_string()));
        mock.expect_fetch_gists().times(1).returning(|_| Ok(vec![]));

        // verboseモードで実行
        let updater = CacheUpdater::new_with_client(config, true, mock);
        let result = updater.update(true);

        assert!(result.is_ok());
    }

    #[test]
    fn test_update_gist_modification_verbose() {
        let config = create_test_config();
        config.ensure_cache_dir().unwrap();

        // 既存のキャッシュを作成
        let old_time = Utc::now() - chrono::Duration::hours(1);
        let existing_cache = GistCache {
            metadata: CacheMetadata {
                last_updated: old_time,
                total_count: 1,
                github_user: "testuser".to_string(),
            },
            gists: vec![GistInfo {
                id: "modified123".to_string(),
                description: Some("Old version".to_string()),
                files: vec![crate::cache::types::GistFile {
                    filename: "test.sh".to_string(),
                    language: Some("Shell".to_string()),
                    size: 100,
                }],
                updated_at: old_time,
                public: true,
                html_url: "https://gist.github.com/modified123".to_string(),
            }],
        };

        let updater_temp = CacheUpdater::new(config.clone(), false);
        updater_temp.save_cache(&existing_cache).unwrap();

        // コンテンツキャッシュを作成
        let content_cache = ContentCache::new(config.contents_dir.clone());
        content_cache.ensure_cache_dir().unwrap();
        content_cache
            .write("modified123", "test.sh", "echo old")
            .unwrap();

        let mut mock = MockGitHubClient::new();

        mock.expect_check_auth().times(1).returning(|| Ok(()));
        mock.expect_check_rate_limit()
            .times(1)
            .returning(|| Ok(4000));

        // 更新されたGistを返す
        let new_time = Utc::now();
        mock.expect_fetch_gists().times(1).returning(move |_| {
            Ok(vec![crate::cache::types::GitHubGist {
                id: "modified123".to_string(),
                description: Some("New version".to_string()),
                files: HashMap::from([(
                    "test.sh".to_string(),
                    GitHubFile {
                        filename: "test.sh".to_string(),
                        language: Some("Shell".to_string()),
                        size: 150,
                    },
                )]),
                updated_at: new_time,
                public: true,
                html_url: "https://gist.github.com/modified123".to_string(),
            }])
        });

        // verboseモードで実行して、Gist更新検出ログをカバー
        let updater = CacheUpdater::new_with_client(config.clone(), true, mock);
        let result = updater.update(false);

        assert!(result.is_ok());

        // キャッシュが削除されたことを確認
        assert!(!content_cache.exists("modified123", "test.sh"));
    }

    #[test]
    fn test_update_force_verbose_without_existing_cache() {
        let config = create_test_config();
        config.ensure_cache_dir().unwrap();

        let mut mock = MockGitHubClient::new();

        mock.expect_check_auth().times(1).returning(|| Ok(()));
        mock.expect_check_rate_limit()
            .times(1)
            .returning(|| Ok(5000));
        mock.expect_get_user()
            .times(1)
            .returning(|| Ok("newuser".to_string()));
        mock.expect_fetch_gists().times(1).returning(|_| {
            Ok(vec![crate::cache::types::GitHubGist {
                id: "new123".to_string(),
                description: Some("New gist".to_string()),
                files: HashMap::from([(
                    "new.sh".to_string(),
                    GitHubFile {
                        filename: "new.sh".to_string(),
                        language: Some("Shell".to_string()),
                        size: 100,
                    },
                )]),
                updated_at: Utc::now(),
                public: true,
                html_url: "https://gist.github.com/new123".to_string(),
            }])
        });

        // verboseモードで強制更新（既存キャッシュなし）
        let updater = CacheUpdater::new_with_client(config, true, mock);
        let result = updater.update(true);

        assert!(result.is_ok());
    }
}
