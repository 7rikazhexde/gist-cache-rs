use crate::cache::types::{CacheMetadata, GistCache, GistInfo};
use crate::config::Config;
use crate::error::Result;
use crate::github::GitHubApi;
use chrono::Utc;
use colored::Colorize;
use std::collections::HashMap;
use std::fs;

pub struct CacheUpdater {
    config: Config,
    verbose: bool,
}

impl CacheUpdater {
    pub fn new(config: Config, verbose: bool) -> Self {
        Self { config, verbose }
    }

    pub fn update(&self, force: bool) -> Result<()> {
        println!("{}", "Gistキャッシュを更新しています...".cyan());

        if self.verbose {
            if force {
                println!("{}", "モード: 強制全件更新".yellow());
            } else {
                println!("{}", "モード: 差分更新".yellow());
            }
        }

        // Ensure cache directory exists
        self.config.ensure_cache_dir()?;

        // Check authentication
        GitHubApi::check_auth()?;

        // Check rate limit
        let rate_limit = GitHubApi::check_rate_limit()?;
        if rate_limit < 100 {
            println!(
                "{}",
                format!("警告: レートリミット残量が{}と低いです", rate_limit).yellow()
            );
        }
        if self.verbose {
            println!("{}", format!("レートリミット残量: {}", rate_limit).green());
        }

        // Load existing cache if available
        let (github_user, last_updated, old_gists) = if self.config.cache_exists() && !force {
            let cache = self.load_cache()?;
            if self.verbose {
                println!("{}", "既存のキャッシュを検出しました".green());
                println!(
                    "{}",
                    format!(
                        "GitHubユーザー（キャッシュ再利用）: {}",
                        cache.metadata.github_user
                    )
                    .green()
                );
                println!(
                    "{}",
                    format!(
                        "最終更新日時: {}",
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
            (GitHubApi::get_user()?, None, None)
        };

        if self.verbose && last_updated.is_none() {
            println!("{}", format!("GitHubユーザー: {}", github_user).green());
        }

        // Fetch gists from GitHub
        if self.verbose {
            println!("{}", "GitHub APIからGist情報を取得中...".cyan());
        }

        let since = if force { None } else { last_updated };
        let fetched_gists = GitHubApi::fetch_gists(since)?;
        let fetched_count = fetched_gists.len();

        if self.verbose {
            println!("{}", format!("取得したGist数: {}", fetched_count).green());
        }

        // Merge with existing cache if doing differential update
        let final_gists = if let Some(mut old) = old_gists {
            if fetched_count == 0 {
                println!("{}", "更新なし".green());
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
                            "差分マージ完了: 既存 {} + 差分 {} → 総数 {}",
                            old_count, fetched_count, total_count
                        )
                        .green()
                    );
                }

                println!("{}", format!("更新: {}件", fetched_count).green());
                if new > 0 && self.verbose {
                    println!("{}", format!("新規Gist: {}件", new).green());
                }

                merged
            }
        } else {
            // Force update or first time
            let mut gists: Vec<GistInfo> = fetched_gists.into_iter().map(GistInfo::from).collect();

            // Sort by updated_at descending
            gists.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));

            let count = gists.len();
            println!("{}", format!("新規/更新: {}件", count).green());
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

        println!("{}", "キャッシュ更新が完了しました".green().bold());
        println!(
            "{}",
            format!("総Gist数: {}", cache.metadata.total_count)
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
