use crate::cache::types::GistCache;
use crate::error::{GistCacheError, Result};
use chrono::{Duration, Utc};
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

/// Structure for managing Gist content cache
pub struct ContentCache {
    cache_dir: PathBuf,
}

impl ContentCache {
    /// Create a new ContentCache instance
    ///
    /// # Arguments
    /// * `cache_dir` - Cache directory path (~/.cache/gist-cache/contents)
    pub fn new(cache_dir: PathBuf) -> Self {
        Self { cache_dir }
    }

    /// Ensure cache directory exists
    pub fn ensure_cache_dir(&self) -> Result<()> {
        if !self.cache_dir.exists() {
            fs::create_dir_all(&self.cache_dir)?;
        }
        Ok(())
    }

    /// Get cache file path for a Gist
    ///
    /// # Arguments
    /// * `gist_id` - Gist ID
    /// * `filename` - File name
    ///
    /// # Returns
    /// Full path to cache file
    fn get_cache_path(&self, gist_id: &str, filename: &str) -> PathBuf {
        self.cache_dir.join(gist_id).join(filename)
    }

    /// Get cache directory path for a Gist
    ///
    /// # Arguments
    /// * `gist_id` - Gist ID
    ///
    /// # Returns
    /// Full path to cache directory
    fn get_gist_dir(&self, gist_id: &str) -> PathBuf {
        self.cache_dir.join(gist_id)
    }

    /// Check if cache file exists
    ///
    /// # Arguments
    /// * `gist_id` - Gist ID
    /// * `filename` - File name
    ///
    /// # Returns
    /// True if cache exists
    pub fn exists(&self, gist_id: &str, filename: &str) -> bool {
        self.get_cache_path(gist_id, filename).exists()
    }

    /// Read content from cache
    ///
    /// # Arguments
    /// * `gist_id` - Gist ID
    /// * `filename` - File name
    ///
    /// # Returns
    /// File content (string)
    ///
    /// # Errors
    /// Returns error if file doesn't exist or read fails
    ///
    /// # Self-healing Principle
    /// If cache file is corrupted, return error instead of panicking,
    /// allowing caller to re-fetch from API
    pub fn read(&self, gist_id: &str, filename: &str) -> Result<String> {
        let path = self.get_cache_path(gist_id, filename);

        if !path.exists() {
            return Err(GistCacheError::CacheReadError(format!(
                "Cache file not found: {}",
                path.display()
            )));
        }

        fs::read_to_string(&path).map_err(|e| {
            GistCacheError::CacheReadError(format!(
                "Failed to read cache file {}: {}",
                path.display(),
                e
            ))
        })
    }

    /// Write content to cache file
    ///
    /// # Arguments
    /// * `gist_id` - Gist ID
    /// * `filename` - File name
    /// * `content` - File content
    ///
    /// # Errors
    /// Returns error if directory creation or file write fails
    ///
    /// # Implementation Details
    /// - Atomic write (temp file → rename) to avoid concurrent access conflicts
    /// - Automatically creates Gist directory if it doesn't exist
    pub fn write(&self, gist_id: &str, filename: &str, content: &str) -> Result<()> {
        let gist_dir = self.get_gist_dir(gist_id);
        let cache_path = self.get_cache_path(gist_id, filename);

        // Create Gist directory
        if !gist_dir.exists() {
            fs::create_dir_all(&gist_dir).map_err(|e| {
                GistCacheError::CacheWriteError(format!(
                    "Failed to create gist directory {}: {}",
                    gist_dir.display(),
                    e
                ))
            })?;
        }

        // Atomic write: temp file → rename
        let temp_path = cache_path.with_extension("tmp");

        // Write to temp file
        fs::write(&temp_path, content).map_err(|e| {
            GistCacheError::CacheWriteError(format!(
                "Failed to write temp file {}: {}",
                temp_path.display(),
                e
            ))
        })?;

        // Atomically replace file with rename
        fs::rename(&temp_path, &cache_path).map_err(|e| {
            // Remove temp file if failed
            let _ = fs::remove_file(&temp_path);
            GistCacheError::CacheWriteError(format!(
                "Failed to rename temp file to cache file {}: {}",
                cache_path.display(),
                e
            ))
        })?;

        Ok(())
    }

    /// Delete cache for a specific Gist
    ///
    /// # Arguments
    /// * `gist_id` - Gist ID
    ///
    /// # Returns
    /// `Ok(true)` if actually deleted, `Ok(false)` if didn't exist
    ///
    /// # Errors
    /// Returns error if deletion fails (but not if directory doesn't exist)
    pub fn delete_gist(&self, gist_id: &str) -> Result<bool> {
        let gist_dir = self.get_gist_dir(gist_id);

        if !gist_dir.exists() {
            // Skip if directory doesn't exist (not an error)
            return Ok(false); // Not deleted
        }

        // Self-healing principle: Delete entire directory
        // Even if unexpected files exist, delete the whole directory to restore normal state
        fs::remove_dir_all(&gist_dir).map_err(|e| {
            GistCacheError::CacheDeleteError(format!(
                "Failed to delete gist cache directory {}: {}",
                gist_dir.display(),
                e
            ))
        })?;

        Ok(true) // Deleted
    }

    /// Get all cached Gist IDs
    ///
    /// # Returns
    /// List of Gist IDs
    ///
    /// # Self-healing Principle
    /// Ignore unexpected files or directories and return only valid Gist IDs
    pub fn list_cached_gists(&self) -> Result<Vec<String>> {
        if !self.cache_dir.exists() {
            return Ok(Vec::new());
        }

        let mut gist_ids = Vec::new();

        for entry in fs::read_dir(&self.cache_dir)? {
            let entry = match entry {
                Ok(e) => e,
                Err(_) => continue, // Ignore read errors
            };

            let path = entry.path();

            // Only target directories
            if path.is_dir()
                && let Some(gist_id) = path.file_name().and_then(|n| n.to_str())
            {
                gist_ids.push(gist_id.to_string());
            }
        }

        Ok(gist_ids)
    }

    /// Calculate total cache size (in bytes)
    ///
    /// # Returns
    /// Size of entire cache directory
    pub fn total_size(&self) -> Result<u64> {
        if !self.cache_dir.exists() {
            return Ok(0);
        }

        let mut total_size = 0u64;

        fn calculate_dir_size(path: &Path, total: &mut u64) -> std::io::Result<()> {
            for entry in fs::read_dir(path)? {
                let entry = entry?;
                let path = entry.path();

                if path.is_file() {
                    if let Ok(metadata) = fs::metadata(&path) {
                        *total += metadata.len();
                    }
                } else if path.is_dir() {
                    calculate_dir_size(&path, total)?;
                }
            }
            Ok(())
        }

        calculate_dir_size(&self.cache_dir, &mut total_size)?;

        Ok(total_size)
    }

    /// Clear all cache
    ///
    /// # Errors
    /// Returns error if deletion fails
    pub fn clear_all(&self) -> Result<()> {
        if !self.cache_dir.exists() {
            return Ok(());
        }

        fs::remove_dir_all(&self.cache_dir).map_err(|e| {
            GistCacheError::CacheDeleteError(format!(
                "Failed to clear cache directory {}: {}",
                self.cache_dir.display(),
                e
            ))
        })?;

        // Recreate directory
        self.ensure_cache_dir()?;

        Ok(())
    }

    /// Clean old cache entries
    ///
    /// # Arguments
    /// * `metadata_cache` - Metadata cache to check against
    /// * `options` - Clean options
    ///
    /// # Returns
    /// Result containing deleted gists and their total size
    pub fn clean(&self, metadata_cache: &GistCache, options: &CleanOptions) -> Result<CleanResult> {
        let mut result = CleanResult {
            deleted_gists: Vec::new(),
            deleted_size: 0,
            orphaned_files: Vec::new(),
        };

        // Create set of valid gist IDs from metadata
        let valid_gist_ids: HashSet<String> =
            metadata_cache.gists.iter().map(|g| g.id.clone()).collect();

        // Get all cached gist IDs
        let cached_gist_ids = self.list_cached_gists()?;

        // Process each cached gist
        for gist_id in &cached_gist_ids {
            let should_delete =
                self.should_delete_gist(gist_id, &valid_gist_ids, metadata_cache, options)?;

            if should_delete {
                // Calculate size before deletion
                let gist_dir = self.get_gist_dir(gist_id);
                let size = self.calculate_dir_size(&gist_dir)?;
                result.deleted_size += size;
                result.deleted_gists.push(gist_id.clone());

                // Actually delete if not dry run
                if !options.dry_run {
                    self.delete_gist(gist_id)?;
                }
            }
        }

        Ok(result)
    }

    /// Check if a gist should be deleted based on clean options
    fn should_delete_gist(
        &self,
        gist_id: &str,
        valid_gist_ids: &HashSet<String>,
        metadata_cache: &GistCache,
        options: &CleanOptions,
    ) -> Result<bool> {
        // Check for orphaned (not in metadata)
        if options.orphaned && !valid_gist_ids.contains(gist_id) {
            return Ok(true);
        }

        // Check for old entries
        if let Some(days) = options.older_than_days {
            if let Some(gist_info) = metadata_cache.gists.iter().find(|g| g.id == gist_id) {
                let cutoff_date = Utc::now() - Duration::days(days as i64);
                if gist_info.updated_at < cutoff_date {
                    return Ok(true);
                }
            }
        }

        Ok(false)
    }

    /// Calculate size of a directory
    fn calculate_dir_size(&self, path: &Path) -> Result<u64> {
        let mut total_size = 0u64;

        if !path.exists() {
            return Ok(0);
        }

        fn calc_recursive(path: &Path, total: &mut u64) -> std::io::Result<()> {
            for entry in fs::read_dir(path)? {
                let entry = entry?;
                let path = entry.path();

                if path.is_file() {
                    if let Ok(metadata) = fs::metadata(&path) {
                        *total += metadata.len();
                    }
                } else if path.is_dir() {
                    calc_recursive(&path, total)?;
                }
            }
            Ok(())
        }

        calc_recursive(path, &mut total_size)?;
        Ok(total_size)
    }
}

/// Options for cache cleaning
#[derive(Debug, Clone, Default)]
pub struct CleanOptions {
    /// Remove entries older than specified days
    pub older_than_days: Option<u32>,
    /// Remove orphaned content cache files
    pub orphaned: bool,
    /// Preview mode (don't actually delete)
    pub dry_run: bool,
}

/// Result of cache cleaning operation
#[derive(Debug, Clone, Default)]
pub struct CleanResult {
    /// List of deleted gist IDs
    pub deleted_gists: Vec<String>,
    /// Total size of deleted data in bytes
    pub deleted_size: u64,
    /// List of orphaned files (for information)
    pub orphaned_files: Vec<PathBuf>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn setup_test_cache() -> (TempDir, ContentCache) {
        let temp_dir = TempDir::new().unwrap();
        let cache_dir = temp_dir.path().join("contents");
        let cache = ContentCache::new(cache_dir);
        cache.ensure_cache_dir().unwrap();
        (temp_dir, cache)
    }

    #[test]
    fn test_write_and_read() {
        let (_temp, cache) = setup_test_cache();

        let gist_id = "test123";
        let filename = "test.sh";
        let content = "#!/bin/bash\necho hello";

        // 書き込み
        cache.write(gist_id, filename, content).unwrap();

        // 存在確認
        assert!(cache.exists(gist_id, filename));

        // 読み込み
        let read_content = cache.read(gist_id, filename).unwrap();
        assert_eq!(read_content, content);
    }

    #[test]
    fn test_delete_gist() {
        let (_temp, cache) = setup_test_cache();

        let gist_id = "test456";
        let filename = "test.py";
        let content = "print('hello')";

        // 書き込み
        cache.write(gist_id, filename, content).unwrap();
        assert!(cache.exists(gist_id, filename));

        // 削除
        let deleted = cache.delete_gist(gist_id).unwrap();
        assert!(deleted); // 削除されたことを確認
        assert!(!cache.exists(gist_id, filename));

        // 存在しないGistの削除はエラーにならず、false を返す
        let deleted = cache.delete_gist("nonexistent").unwrap();
        assert!(!deleted); // 削除されなかったことを確認
    }

    #[test]
    fn test_list_cached_gists() {
        let (_temp, cache) = setup_test_cache();

        // 複数のGistを書き込み
        cache.write("gist1", "file1.sh", "content1").unwrap();
        cache.write("gist2", "file2.py", "content2").unwrap();
        cache.write("gist3", "file3.rb", "content3").unwrap();

        let gist_ids = cache.list_cached_gists().unwrap();
        assert_eq!(gist_ids.len(), 3);
        assert!(gist_ids.contains(&"gist1".to_string()));
        assert!(gist_ids.contains(&"gist2".to_string()));
        assert!(gist_ids.contains(&"gist3".to_string()));
    }

    #[test]
    fn test_self_healing_unexpected_files() {
        let (_temp, cache) = setup_test_cache();

        let gist_id = "test789";
        let filename = "test.sh";
        let content = "#!/bin/bash\necho test";

        // 通常のキャッシュを作成
        cache.write(gist_id, filename, content).unwrap();

        // 手動で予期しないファイルを追加（自己修復テスト）
        let gist_dir = cache.get_gist_dir(gist_id);
        fs::write(gist_dir.join("unexpected.txt"), "unexpected content").unwrap();

        // ディレクトリ内に2つのファイルが存在する状態
        let entries: Vec<_> = fs::read_dir(&gist_dir).unwrap().collect();
        assert_eq!(entries.len(), 2);

        // delete_gistはディレクトリ全体を削除（自己修復）
        let deleted = cache.delete_gist(gist_id).unwrap();
        assert!(deleted); // 実際に削除されたことを確認

        // ディレクトリ全体が削除されていることを確認
        assert!(!gist_dir.exists());
    }

    #[test]
    fn test_total_size() {
        let (_temp, cache) = setup_test_cache();

        // 初期状態は0
        assert_eq!(cache.total_size().unwrap(), 0);

        // いくつかのファイルを書き込み
        cache.write("gist1", "file1.sh", "content1").unwrap();
        cache
            .write("gist2", "file2.py", "longer content 2")
            .unwrap();

        // サイズが増加していることを確認
        let size = cache.total_size().unwrap();
        assert!(size > 0);
    }

    #[test]
    fn test_clear_all() {
        let (_temp, cache) = setup_test_cache();

        // いくつかのキャッシュを作成
        cache.write("gist1", "file1.sh", "content1").unwrap();
        cache.write("gist2", "file2.py", "content2").unwrap();

        // クリア前は2つのGistが存在
        assert_eq!(cache.list_cached_gists().unwrap().len(), 2);

        // すべてクリア
        cache.clear_all().unwrap();

        // クリア後は0個
        assert_eq!(cache.list_cached_gists().unwrap().len(), 0);
    }

    #[test]
    fn test_read_nonexistent_file() {
        let (_temp, cache) = setup_test_cache();

        // 存在しないファイルを読み込もうとするとエラー
        let result = cache.read("nonexistent", "file.sh");
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            GistCacheError::CacheReadError(_)
        ));
    }

    #[test]
    fn test_clear_all_when_empty() {
        let temp_dir = TempDir::new().unwrap();
        let cache_dir = temp_dir.path().join("contents");
        let cache = ContentCache::new(cache_dir);

        // キャッシュディレクトリが存在しない状態でclear_all
        cache.clear_all().unwrap(); // エラーにならない
    }

    #[test]
    fn test_total_size_when_no_cache_dir() {
        let temp_dir = TempDir::new().unwrap();
        let cache_dir = temp_dir.path().join("nonexistent_contents");
        let cache = ContentCache::new(cache_dir);

        // キャッシュディレクトリが存在しない場合は0を返す
        assert_eq!(cache.total_size().unwrap(), 0);
    }

    #[test]
    fn test_list_cached_gists_when_no_cache_dir() {
        let temp_dir = TempDir::new().unwrap();
        let cache_dir = temp_dir.path().join("nonexistent_contents");
        let cache = ContentCache::new(cache_dir);

        // キャッシュディレクトリが存在しない場合は空のベクトルを返す
        assert_eq!(cache.list_cached_gists().unwrap().len(), 0);
    }

    #[test]
    fn test_write_creates_gist_directory() {
        let temp_dir = TempDir::new().unwrap();
        let cache_dir = temp_dir.path().join("contents");
        let cache = ContentCache::new(cache_dir.clone());

        // ensure_cache_dirを呼ばずに直接書き込み
        cache.write("new_gist", "file.sh", "content").unwrap();

        // ディレクトリとファイルが自動作成されることを確認
        assert!(cache_dir.join("new_gist").exists());
        assert!(cache.exists("new_gist", "file.sh"));
    }

    #[test]
    fn test_list_cached_gists_with_file_in_contents_dir() {
        let (temp, cache) = setup_test_cache();
        let cache_dir = temp.path().join("contents");

        // 通常のGistキャッシュを作成
        cache.write("gist1", "file1.sh", "content1").unwrap();

        // contentsディレクトリ直下に予期しないファイルを作成
        let unexpected_file = cache_dir.join("unexpected_file.txt");
        fs::write(&unexpected_file, "unexpected").unwrap();

        // list_cached_gistsは通常のGistのみを返す（ファイルは無視される）
        let gist_ids = cache.list_cached_gists().unwrap();
        assert_eq!(gist_ids.len(), 1);
        assert!(gist_ids.contains(&"gist1".to_string()));
    }

    #[test]
    fn test_delete_gist_already_deleted() {
        let (_temp, cache) = setup_test_cache();

        let gist_id = "test_delete";
        cache.write(gist_id, "file.sh", "content").unwrap();

        // 最初の削除は成功
        assert!(cache.delete_gist(gist_id).unwrap());

        // 2回目の削除はfalseを返す（既に存在しない）
        assert!(!cache.delete_gist(gist_id).unwrap());
    }

    #[test]
    fn test_multiple_files_in_same_gist() {
        let (_temp, cache) = setup_test_cache();

        let gist_id = "multi_file_gist";

        // 同じGistに複数のファイルを書き込み
        cache.write(gist_id, "file1.sh", "content1").unwrap();
        cache.write(gist_id, "file2.py", "content2").unwrap();
        cache.write(gist_id, "file3.rb", "content3").unwrap();

        // 全てのファイルが存在することを確認
        assert!(cache.exists(gist_id, "file1.sh"));
        assert!(cache.exists(gist_id, "file2.py"));
        assert!(cache.exists(gist_id, "file3.rb"));

        // Gistを削除すると全ファイルが削除される
        assert!(cache.delete_gist(gist_id).unwrap());
        assert!(!cache.exists(gist_id, "file1.sh"));
        assert!(!cache.exists(gist_id, "file2.py"));
        assert!(!cache.exists(gist_id, "file3.rb"));
    }

    #[test]
    fn test_overwrite_existing_file() {
        let (_temp, cache) = setup_test_cache();

        let gist_id = "overwrite_test";
        let filename = "file.sh";

        // 最初の書き込み
        cache.write(gist_id, filename, "original content").unwrap();
        assert_eq!(cache.read(gist_id, filename).unwrap(), "original content");

        // 上書き
        cache.write(gist_id, filename, "new content").unwrap();
        assert_eq!(cache.read(gist_id, filename).unwrap(), "new content");
    }

    #[test]
    fn test_cache_path_generation() {
        let temp_dir = TempDir::new().unwrap();
        let cache_dir = temp_dir.path().join("contents");
        let cache = ContentCache::new(cache_dir.clone());

        let path = cache.get_cache_path("test_id", "test_file.sh");
        assert_eq!(path, cache_dir.join("test_id").join("test_file.sh"));
    }

    #[test]
    fn test_clean_with_older_than() {
        use crate::cache::types::{CacheMetadata, GistCache, GistFile, GistInfo};
        use chrono::{Duration, Utc};

        let (_temp, cache) = setup_test_cache();

        // Create old and new gists
        let old_date = Utc::now() - Duration::days(40);
        let new_date = Utc::now() - Duration::days(10);

        let old_gist = GistInfo {
            id: "old_gist".to_string(),
            description: Some("Old gist".to_string()),
            files: vec![GistFile {
                filename: "old.sh".to_string(),
                language: Some("Shell".to_string()),
                size: 100,
            }],
            updated_at: old_date,
            public: true,
            html_url: "https://gist.github.com/old_gist".to_string(),
        };

        let new_gist = GistInfo {
            id: "new_gist".to_string(),
            description: Some("New gist".to_string()),
            files: vec![GistFile {
                filename: "new.sh".to_string(),
                language: Some("Shell".to_string()),
                size: 100,
            }],
            updated_at: new_date,
            public: true,
            html_url: "https://gist.github.com/new_gist".to_string(),
        };

        let metadata_cache = GistCache {
            metadata: CacheMetadata {
                last_updated: Utc::now(),
                total_count: 2,
                github_user: "testuser".to_string(),
            },
            gists: vec![old_gist, new_gist],
        };

        // Write both to cache
        cache.write("old_gist", "old.sh", "echo old").unwrap();
        cache.write("new_gist", "new.sh", "echo new").unwrap();

        // Clean entries older than 30 days
        let options = CleanOptions {
            older_than_days: Some(30),
            orphaned: false,
            dry_run: false,
        };

        let result = cache.clean(&metadata_cache, &options).unwrap();

        // Only old gist should be deleted
        assert_eq!(result.deleted_gists.len(), 1);
        assert!(result.deleted_gists.contains(&"old_gist".to_string()));
        assert!(!cache.exists("old_gist", "old.sh"));
        assert!(cache.exists("new_gist", "new.sh"));
    }

    #[test]
    fn test_clean_with_orphaned() {
        use crate::cache::types::{CacheMetadata, GistCache, GistFile, GistInfo};
        use chrono::Utc;

        let (_temp, cache) = setup_test_cache();

        // Create metadata cache with only one gist
        let valid_gist = GistInfo {
            id: "valid_gist".to_string(),
            description: Some("Valid gist".to_string()),
            files: vec![GistFile {
                filename: "valid.sh".to_string(),
                language: Some("Shell".to_string()),
                size: 100,
            }],
            updated_at: Utc::now(),
            public: true,
            html_url: "https://gist.github.com/valid_gist".to_string(),
        };

        let metadata_cache = GistCache {
            metadata: CacheMetadata {
                last_updated: Utc::now(),
                total_count: 1,
                github_user: "testuser".to_string(),
            },
            gists: vec![valid_gist],
        };

        // Write valid gist and orphaned gist to cache
        cache.write("valid_gist", "valid.sh", "echo valid").unwrap();
        cache
            .write("orphaned_gist", "orphaned.sh", "echo orphaned")
            .unwrap();

        // Clean orphaned entries
        let options = CleanOptions {
            older_than_days: None,
            orphaned: true,
            dry_run: false,
        };

        let result = cache.clean(&metadata_cache, &options).unwrap();

        // Only orphaned gist should be deleted
        assert_eq!(result.deleted_gists.len(), 1);
        assert!(result.deleted_gists.contains(&"orphaned_gist".to_string()));
        assert!(cache.exists("valid_gist", "valid.sh"));
        assert!(!cache.exists("orphaned_gist", "orphaned.sh"));
    }

    #[test]
    fn test_clean_with_dry_run() {
        use crate::cache::types::{CacheMetadata, GistCache, GistFile, GistInfo};
        use chrono::{Duration, Utc};

        let (_temp, cache) = setup_test_cache();

        let old_gist = GistInfo {
            id: "old_gist".to_string(),
            description: Some("Old gist".to_string()),
            files: vec![GistFile {
                filename: "old.sh".to_string(),
                language: Some("Shell".to_string()),
                size: 100,
            }],
            updated_at: Utc::now() - Duration::days(40),
            public: true,
            html_url: "https://gist.github.com/old_gist".to_string(),
        };

        let metadata_cache = GistCache {
            metadata: CacheMetadata {
                last_updated: Utc::now(),
                total_count: 1,
                github_user: "testuser".to_string(),
            },
            gists: vec![old_gist],
        };

        cache.write("old_gist", "old.sh", "echo old").unwrap();

        // Dry run with older_than
        let options = CleanOptions {
            older_than_days: Some(30),
            orphaned: false,
            dry_run: true,
        };

        let result = cache.clean(&metadata_cache, &options).unwrap();

        // Should report what would be deleted
        assert_eq!(result.deleted_gists.len(), 1);
        assert!(result.deleted_gists.contains(&"old_gist".to_string()));
        // But file should still exist
        assert!(cache.exists("old_gist", "old.sh"));
    }

    #[test]
    fn test_clean_with_no_criteria() {
        use crate::cache::types::{CacheMetadata, GistCache, GistFile, GistInfo};
        use chrono::Utc;

        let (_temp, cache) = setup_test_cache();

        let gist = GistInfo {
            id: "test_gist".to_string(),
            description: Some("Test gist".to_string()),
            files: vec![GistFile {
                filename: "test.sh".to_string(),
                language: Some("Shell".to_string()),
                size: 100,
            }],
            updated_at: Utc::now(),
            public: true,
            html_url: "https://gist.github.com/test_gist".to_string(),
        };

        let metadata_cache = GistCache {
            metadata: CacheMetadata {
                last_updated: Utc::now(),
                total_count: 1,
                github_user: "testuser".to_string(),
            },
            gists: vec![gist],
        };

        cache.write("test_gist", "test.sh", "echo test").unwrap();

        // No criteria specified
        let options = CleanOptions {
            older_than_days: None,
            orphaned: false,
            dry_run: false,
        };

        let result = cache.clean(&metadata_cache, &options).unwrap();

        // Nothing should be deleted
        assert_eq!(result.deleted_gists.len(), 0);
        assert!(cache.exists("test_gist", "test.sh"));
    }

    #[test]
    fn test_clean_with_both_criteria() {
        use crate::cache::types::{CacheMetadata, GistCache, GistFile, GistInfo};
        use chrono::{Duration, Utc};

        let (_temp, cache) = setup_test_cache();

        // Create an old gist (in metadata)
        let old_gist = GistInfo {
            id: "old_gist".to_string(),
            description: Some("Old gist".to_string()),
            files: vec![GistFile {
                filename: "old.sh".to_string(),
                language: Some("Shell".to_string()),
                size: 100,
            }],
            updated_at: Utc::now() - Duration::days(40),
            public: true,
            html_url: "https://gist.github.com/old_gist".to_string(),
        };

        let metadata_cache = GistCache {
            metadata: CacheMetadata {
                last_updated: Utc::now(),
                total_count: 1,
                github_user: "testuser".to_string(),
            },
            gists: vec![old_gist],
        };

        // Write old gist and orphaned gist to cache
        cache.write("old_gist", "old.sh", "echo old").unwrap();
        cache
            .write("orphaned_gist", "orphaned.sh", "echo orphaned")
            .unwrap();

        // Clean with both criteria
        let options = CleanOptions {
            older_than_days: Some(30),
            orphaned: true,
            dry_run: false,
        };

        let result = cache.clean(&metadata_cache, &options).unwrap();

        // Both old and orphaned gists should be deleted
        assert_eq!(result.deleted_gists.len(), 2);
        assert!(result.deleted_gists.contains(&"old_gist".to_string()));
        assert!(result.deleted_gists.contains(&"orphaned_gist".to_string()));
        assert!(!cache.exists("old_gist", "old.sh"));
        assert!(!cache.exists("orphaned_gist", "orphaned.sh"));
    }

    #[test]
    fn test_clean_when_nothing_matches() {
        use crate::cache::types::{CacheMetadata, GistCache, GistFile, GistInfo};
        use chrono::Utc;

        let (_temp, cache) = setup_test_cache();

        let gist = GistInfo {
            id: "recent_gist".to_string(),
            description: Some("Recent gist".to_string()),
            files: vec![GistFile {
                filename: "recent.sh".to_string(),
                language: Some("Shell".to_string()),
                size: 100,
            }],
            updated_at: Utc::now(),
            public: true,
            html_url: "https://gist.github.com/recent_gist".to_string(),
        };

        let metadata_cache = GistCache {
            metadata: CacheMetadata {
                last_updated: Utc::now(),
                total_count: 1,
                github_user: "testuser".to_string(),
            },
            gists: vec![gist],
        };

        cache
            .write("recent_gist", "recent.sh", "echo recent")
            .unwrap();

        // Try to clean entries older than 30 days (but there are none)
        let options = CleanOptions {
            older_than_days: Some(30),
            orphaned: false,
            dry_run: false,
        };

        let result = cache.clean(&metadata_cache, &options).unwrap();

        // Nothing should be deleted
        assert_eq!(result.deleted_gists.len(), 0);
        assert_eq!(result.deleted_size, 0);
        assert!(cache.exists("recent_gist", "recent.sh"));
    }

    #[test]
    fn test_calculate_dir_size() {
        let (_temp, cache) = setup_test_cache();

        // Write files with known sizes
        cache.write("size_test", "file1.sh", "12345").unwrap();
        cache.write("size_test", "file2.sh", "1234567890").unwrap();

        let gist_dir = cache.get_gist_dir("size_test");
        let size = cache.calculate_dir_size(&gist_dir).unwrap();

        // Size should be the sum of file contents
        // file1.sh: 5 bytes, file2.sh: 10 bytes
        assert_eq!(size, 15);
    }
}
