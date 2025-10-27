use thiserror::Error;

#[derive(Error, Debug)]
pub enum GistCacheError {
    #[error("GitHub API error: {0}")]
    GitHubApi(String),

    #[error("Cache file not found. Please run 'gist-cache-rs update' first")]
    CacheNotFound,

    #[error("Failed to parse cache file: {0}")]
    CacheParse(#[from] serde_json::Error),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("HTTP request error: {0}")]
    Request(#[from] reqwest::Error),

    #[error("Gist not found: {0}")]
    GistNotFound(String),

    #[error("No search results for query: {0}")]
    NoSearchResults(String),

    #[error("Invalid selection")]
    InvalidSelection,

    #[error("GitHub CLI (gh) is not authenticated. Please run 'gh auth login'")]
    NotAuthenticated,

    #[error("Rate limit exceeded. Remaining: {0}")]
    RateLimitExceeded(i64),

    #[error("Execution error: {0}")]
    Execution(String),

    #[error("Invalid interpreter: {0}")]
    InvalidInterpreter(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Failed to read cache file: {0}")]
    CacheReadError(String),

    #[error("Failed to write cache file: {0}")]
    CacheWriteError(String),

    #[error("Failed to delete cache: {0}")]
    CacheDeleteError(String),

    #[error("Cache directory error: {0}")]
    CacheDirectoryError(String),
}

pub type Result<T> = std::result::Result<T, GistCacheError>;
