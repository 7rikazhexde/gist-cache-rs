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

        // Cache directory: follow platform standards
        #[cfg(unix)]
        let cache_dir = home.join(".cache").join("gist-cache");

        #[cfg(windows)]
        let cache_dir = dirs::cache_dir()
            .unwrap_or_else(|| home.join("AppData").join("Local"))
            .join("gist-cache");

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
