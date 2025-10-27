use crate::error::{GistCacheError, Result};
use std::path::PathBuf;

pub struct Config {
    pub cache_dir: PathBuf,
    pub cache_file: PathBuf,
    pub contents_dir: PathBuf,
}

impl Config {
    pub fn new() -> Result<Self> {
        let home = dirs::home_dir()
            .ok_or_else(|| GistCacheError::Config("Could not find home directory".to_string()))?;

        let cache_dir = home.join(".cache").join("gist-cache");
        let cache_file = cache_dir.join("cache.json");
        let contents_dir = cache_dir.join("contents");

        Ok(Self {
            cache_dir,
            cache_file,
            contents_dir,
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

    pub fn cache_exists(&self) -> bool {
        self.cache_file.exists()
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new().expect("Failed to create config")
    }
}
