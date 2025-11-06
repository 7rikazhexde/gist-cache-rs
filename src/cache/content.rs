use crate::error::{GistCacheError, Result};
use std::fs;
use std::path::{Path, PathBuf};

/// Gist本文のキャッシュを管理する構造体
pub struct ContentCache {
    cache_dir: PathBuf,
}

impl ContentCache {
    /// 新しいContentCacheインスタンスを作成
    ///
    /// # Arguments
    /// * `cache_dir` - キャッシュディレクトリのパス（~/.cache/gist-cache/contents）
    pub fn new(cache_dir: PathBuf) -> Self {
        Self { cache_dir }
    }

    /// キャッシュディレクトリが存在することを保証
    pub fn ensure_cache_dir(&self) -> Result<()> {
        if !self.cache_dir.exists() {
            fs::create_dir_all(&self.cache_dir)?;
        }
        Ok(())
    }

    /// Gistのキャッシュファイルパスを取得
    ///
    /// # Arguments
    /// * `gist_id` - GistのID
    /// * `filename` - ファイル名
    ///
    /// # Returns
    /// キャッシュファイルのフルパス
    fn get_cache_path(&self, gist_id: &str, filename: &str) -> PathBuf {
        self.cache_dir.join(gist_id).join(filename)
    }

    /// Gistのキャッシュディレクトリパスを取得
    ///
    /// # Arguments
    /// * `gist_id` - GistのID
    ///
    /// # Returns
    /// キャッシュディレクトリのフルパス
    fn get_gist_dir(&self, gist_id: &str) -> PathBuf {
        self.cache_dir.join(gist_id)
    }

    /// キャッシュファイルが存在するかチェック
    ///
    /// # Arguments
    /// * `gist_id` - GistのID
    /// * `filename` - ファイル名
    ///
    /// # Returns
    /// キャッシュが存在すればtrue
    pub fn exists(&self, gist_id: &str, filename: &str) -> bool {
        self.get_cache_path(gist_id, filename).exists()
    }

    /// キャッシュから本文を読み込む
    ///
    /// # Arguments
    /// * `gist_id` - GistのID
    /// * `filename` - ファイル名
    ///
    /// # Returns
    /// ファイルの内容（文字列）
    ///
    /// # Errors
    /// ファイルが存在しない、または読み込みに失敗した場合
    ///
    /// # 自己修復の原則
    /// キャッシュファイルが破損している場合、エラーを返すのではなく、
    /// 呼び出し側がAPIから再取得できるようにする
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

    /// 本文をキャッシュファイルに書き込む
    ///
    /// # Arguments
    /// * `gist_id` - GistのID
    /// * `filename` - ファイル名
    /// * `content` - ファイルの内容
    ///
    /// # Errors
    /// ディレクトリの作成やファイルの書き込みに失敗した場合
    ///
    /// # 実装の詳細
    /// - 原子的な書き込み（一時ファイル→rename）により同時実行時の競合を回避
    /// - Gistディレクトリが存在しない場合は自動的に作成
    pub fn write(&self, gist_id: &str, filename: &str, content: &str) -> Result<()> {
        let gist_dir = self.get_gist_dir(gist_id);
        let cache_path = self.get_cache_path(gist_id, filename);

        // Gistディレクトリを作成
        if !gist_dir.exists() {
            fs::create_dir_all(&gist_dir).map_err(|e| {
                GistCacheError::CacheWriteError(format!(
                    "Failed to create gist directory {}: {}",
                    gist_dir.display(),
                    e
                ))
            })?;
        }

        // 原子的な書き込み：一時ファイル→rename
        let temp_path = cache_path.with_extension("tmp");

        // 一時ファイルに書き込み
        fs::write(&temp_path, content).map_err(|e| {
            GistCacheError::CacheWriteError(format!(
                "Failed to write temp file {}: {}",
                temp_path.display(),
                e
            ))
        })?;

        // renameで原子的にファイルを置き換え
        fs::rename(&temp_path, &cache_path).map_err(|e| {
            // 失敗した場合は一時ファイルを削除
            let _ = fs::remove_file(&temp_path);
            GistCacheError::CacheWriteError(format!(
                "Failed to rename temp file to cache file {}: {}",
                cache_path.display(),
                e
            ))
        })?;

        Ok(())
    }

    /// 特定のGistのキャッシュを削除
    ///
    /// # Arguments
    /// * `gist_id` - GistのID
    ///
    /// # Returns
    /// 実際に削除した場合は `Ok(true)`、既に存在しなかった場合は `Ok(false)`
    ///
    /// # Errors
    /// 削除に失敗した場合（ただし、ディレクトリが存在しない場合はエラーではない）
    pub fn delete_gist(&self, gist_id: &str) -> Result<bool> {
        let gist_dir = self.get_gist_dir(gist_id);

        if !gist_dir.exists() {
            // ディレクトリが存在しない場合はスキップ（エラーではない）
            return Ok(false); // 削除しなかった
        }

        // 自己修復の原則：ディレクトリ全体を削除
        // 予期しないファイルが存在する場合でも、ディレクトリごと削除して正常な状態に戻す
        fs::remove_dir_all(&gist_dir).map_err(|e| {
            GistCacheError::CacheDeleteError(format!(
                "Failed to delete gist cache directory {}: {}",
                gist_dir.display(),
                e
            ))
        })?;

        Ok(true) // 削除した
    }

    /// キャッシュされているすべてのGist IDを取得
    ///
    /// # Returns
    /// Gist IDのリスト
    ///
    /// # 自己修復の原則
    /// 予期しないファイルやディレクトリは無視し、正常なGist IDのみを返す
    pub fn list_cached_gists(&self) -> Result<Vec<String>> {
        if !self.cache_dir.exists() {
            return Ok(Vec::new());
        }

        let mut gist_ids = Vec::new();

        for entry in fs::read_dir(&self.cache_dir)? {
            let entry = match entry {
                Ok(e) => e,
                Err(_) => continue, // 読み取りエラーは無視
            };

            let path = entry.path();

            // ディレクトリのみを対象
            if path.is_dir()
                && let Some(gist_id) = path.file_name().and_then(|n| n.to_str())
            {
                gist_ids.push(gist_id.to_string());
            }
        }

        Ok(gist_ids)
    }

    /// キャッシュの合計サイズを計算（バイト単位）
    ///
    /// # Returns
    /// キャッシュディレクトリ全体のサイズ
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

    /// すべてのキャッシュをクリア
    ///
    /// # Errors
    /// 削除に失敗した場合
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

        // ディレクトリを再作成
        self.ensure_cache_dir()?;

        Ok(())
    }
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
}
