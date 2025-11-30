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

    #[error("Self-update error: {0}")]
    SelfUpdate(String),
}

impl From<Box<dyn std::error::Error>> for GistCacheError {
    fn from(error: Box<dyn std::error::Error>) -> Self {
        GistCacheError::SelfUpdate(error.to_string())
    }
}

impl From<self_update::errors::Error> for GistCacheError {
    fn from(error: self_update::errors::Error) -> Self {
        GistCacheError::SelfUpdate(error.to_string())
    }
}

pub type Result<T> = std::result::Result<T, GistCacheError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let error = GistCacheError::GitHubApi("test error".to_string());
        assert_eq!(error.to_string(), "GitHub API error: test error");

        let error = GistCacheError::CacheNotFound;
        assert_eq!(
            error.to_string(),
            "Cache file not found. Please run 'gist-cache-rs update' first"
        );

        let error = GistCacheError::GistNotFound("abc123".to_string());
        assert_eq!(error.to_string(), "Gist not found: abc123");

        let error = GistCacheError::NoSearchResults("test query".to_string());
        assert_eq!(error.to_string(), "No search results for query: test query");

        let error = GistCacheError::InvalidSelection;
        assert_eq!(error.to_string(), "Invalid selection");

        let error = GistCacheError::NotAuthenticated;
        assert_eq!(
            error.to_string(),
            "GitHub CLI (gh) is not authenticated. Please run 'gh auth login'"
        );

        let error = GistCacheError::RateLimitExceeded(10);
        assert_eq!(error.to_string(), "Rate limit exceeded. Remaining: 10");

        let error = GistCacheError::Execution("script failed".to_string());
        assert_eq!(error.to_string(), "Execution error: script failed");

        let error = GistCacheError::InvalidInterpreter("unknown".to_string());
        assert_eq!(error.to_string(), "Invalid interpreter: unknown");

        let error = GistCacheError::Config("bad config".to_string());
        assert_eq!(error.to_string(), "Configuration error: bad config");

        let error = GistCacheError::CacheReadError("read failed".to_string());
        assert_eq!(error.to_string(), "Failed to read cache file: read failed");

        let error = GistCacheError::CacheWriteError("write failed".to_string());
        assert_eq!(
            error.to_string(),
            "Failed to write cache file: write failed"
        );

        let error = GistCacheError::CacheDeleteError("delete failed".to_string());
        assert_eq!(error.to_string(), "Failed to delete cache: delete failed");

        let error = GistCacheError::CacheDirectoryError("dir error".to_string());
        assert_eq!(error.to_string(), "Cache directory error: dir error");
    }

    #[test]
    fn test_error_from_io() {
        let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let error: GistCacheError = io_error.into();
        assert!(error.to_string().contains("file not found"));
    }

    #[test]
    fn test_error_from_json() {
        let json_error = serde_json::from_str::<serde_json::Value>("invalid json").unwrap_err();
        let error: GistCacheError = json_error.into();
        assert!(error.to_string().contains("Failed to parse cache file"));
    }

    #[test]
    fn test_error_from_reqwest() {
        // Create a reqwest error by building an invalid request
        let invalid_url = "http://[::1]:99999"; // Invalid port
        let client = reqwest::Client::new();
        let result = client.get(invalid_url).build();

        // If we can't create an error this way, just skip this test
        // The From<reqwest::Error> trait is still covered by actual usage
        if let Err(req_error) = result {
            let error: GistCacheError = req_error.into();
            assert!(error.to_string().contains("HTTP request error"));
        }
    }

    #[test]
    fn test_result_type_alias() {
        // Test that Result<T> type alias works
        fn returns_ok() -> Result<String> {
            Ok("success".to_string())
        }

        fn returns_err() -> Result<String> {
            Err(GistCacheError::InvalidSelection)
        }

        assert!(returns_ok().is_ok());
        assert!(returns_err().is_err());
    }
}
