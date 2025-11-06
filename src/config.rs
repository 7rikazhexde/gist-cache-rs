use crate::error::{GistCacheError, Result};
use std::path::PathBuf;

#[derive(Clone)]
pub struct Config {
    pub cache_dir: PathBuf,
    pub cache_file: PathBuf,
    pub contents_dir: PathBuf,
    pub download_dir: PathBuf,
}

impl Config {
    pub fn new() -> Result<Self> {
        let home = dirs::home_dir()
            .ok_or_else(|| GistCacheError::Config("Could not find home directory".to_string()))?;

        // Cache directory: check for override via environment variable first
        let cache_dir = if let Ok(override_dir) = std::env::var("GIST_CACHE_DIR") {
            PathBuf::from(override_dir).join("gist-cache")
        } else {
            // Cache directory: follow platform standards
            #[cfg(unix)]
            let dir = home.join(".cache").join("gist-cache");

            #[cfg(windows)]
            let dir = dirs::cache_dir()
                .unwrap_or_else(|| home.join("AppData").join("Local"))
                .join("gist-cache");

            dir
        };

        let cache_file = cache_dir.join("cache.json");
        let contents_dir = cache_dir.join("contents");

        // Download directory: use platform standard
        let download_dir = dirs::download_dir().unwrap_or_else(|| home.join("Downloads"));

        Ok(Self {
            cache_dir,
            cache_file,
            contents_dir,
            download_dir,
        })
    }

    pub fn ensure_cache_dir(&self) -> Result<()> {
        if !self.cache_dir.exists() {
            std::fs::create_dir_all(&self.cache_dir)?;
        }
        if !self.contents_dir.exists() {
            std::fs::create_dir_all(&self.contents_dir)?;
        }
        Ok(())
    }

    pub fn ensure_download_dir(&self) -> Result<()> {
        if !self.download_dir.exists() {
            std::fs::create_dir_all(&self.download_dir)?;
        }
        Ok(())
    }

    pub fn cache_exists(&self) -> bool {
        self.cache_file.exists()
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new().expect("Failed to create config")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_config_new() {
        let config = Config::new().unwrap();
        assert!(config.cache_dir.to_string_lossy().contains("gist-cache"));
        assert!(config.cache_file.to_string_lossy().contains("cache.json"));
        assert!(config.contents_dir.to_string_lossy().contains("contents"));
        assert!(!config.download_dir.to_string_lossy().is_empty());
    }

    #[test]
    fn test_config_default() {
        let config = Config::default();
        assert!(config.cache_dir.to_string_lossy().contains("gist-cache"));
    }

    #[test]
    fn test_ensure_cache_dir() {
        let config = Config::new().unwrap();
        let test_cache_dir = std::env::temp_dir().join("test_gist_cache");
        let test_contents_dir = test_cache_dir.join("contents");

        let test_config = Config {
            cache_dir: test_cache_dir.clone(),
            cache_file: test_cache_dir.join("cache.json"),
            contents_dir: test_contents_dir.clone(),
            download_dir: config.download_dir,
        };

        // Clean up if exists
        let _ = fs::remove_dir_all(&test_cache_dir);

        // Ensure directories are created
        test_config.ensure_cache_dir().unwrap();
        assert!(test_cache_dir.exists());
        assert!(test_contents_dir.exists());

        // Clean up
        let _ = fs::remove_dir_all(&test_cache_dir);
    }

    #[test]
    fn test_ensure_download_dir() {
        let config = Config::new().unwrap();
        let test_download_dir = std::env::temp_dir().join("test_downloads");

        let test_config = Config {
            cache_dir: config.cache_dir,
            cache_file: config.cache_file,
            contents_dir: config.contents_dir,
            download_dir: test_download_dir.clone(),
        };

        // Clean up if exists
        let _ = fs::remove_dir_all(&test_download_dir);

        // Ensure directory is created
        test_config.ensure_download_dir().unwrap();
        assert!(test_download_dir.exists());

        // Clean up
        let _ = fs::remove_dir_all(&test_download_dir);
    }

    #[test]
    fn test_cache_exists() {
        let config = Config::new().unwrap();
        let test_cache_dir = std::env::temp_dir().join("test_cache_exists");
        let test_cache_file = test_cache_dir.join("cache.json");

        let test_config = Config {
            cache_dir: test_cache_dir.clone(),
            cache_file: test_cache_file.clone(),
            contents_dir: test_cache_dir.join("contents"),
            download_dir: config.download_dir,
        };

        // Clean up
        let _ = fs::remove_dir_all(&test_cache_dir);
        fs::create_dir_all(&test_cache_dir).unwrap();

        // File doesn't exist
        assert!(!test_config.cache_exists());

        // Create file
        fs::write(&test_cache_file, "{}").unwrap();
        assert!(test_config.cache_exists());

        // Clean up
        let _ = fs::remove_dir_all(&test_cache_dir);
    }
}
