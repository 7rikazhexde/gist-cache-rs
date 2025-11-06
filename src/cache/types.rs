use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// Custom serializer for DateTime to match bash script format (ISO 8601 without subseconds)
mod datetime_format {
    use chrono::{DateTime, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &str = "%Y-%m-%dT%H:%M:%SZ";

    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        DateTime::parse_from_rfc3339(&s)
            .map(|dt| dt.with_timezone(&Utc))
            .map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GistCache {
    pub metadata: CacheMetadata,
    pub gists: Vec<GistInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheMetadata {
    #[serde(with = "datetime_format")]
    pub last_updated: DateTime<Utc>,
    pub total_count: usize,
    pub github_user: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GistInfo {
    pub id: String,
    pub description: Option<String>,
    pub files: Vec<GistFile>,
    #[serde(with = "datetime_format")]
    pub updated_at: DateTime<Utc>,
    pub public: bool,
    pub html_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GistFile {
    pub filename: String,
    pub language: Option<String>,
    pub size: usize,
}

// GitHub API response types
#[derive(Debug, Deserialize, Clone)]
pub struct GitHubGist {
    pub id: String,
    pub description: Option<String>,
    pub files: std::collections::HashMap<String, GitHubFile>,
    pub updated_at: DateTime<Utc>,
    pub public: bool,
    pub html_url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GitHubFile {
    pub filename: String,
    pub language: Option<String>,
    pub size: usize,
}

#[derive(Debug, Deserialize)]
pub struct GitHubUser {
    pub login: String,
}

#[derive(Debug, Deserialize)]
pub struct RateLimit {
    pub resources: RateLimitResources,
}

#[derive(Debug, Deserialize)]
pub struct RateLimitResources {
    pub core: RateLimitCore,
}

#[derive(Debug, Deserialize)]
pub struct RateLimitCore {
    pub remaining: i64,
    pub limit: i64,
}

impl From<GitHubGist> for GistInfo {
    fn from(gh_gist: GitHubGist) -> Self {
        GistInfo {
            id: gh_gist.id,
            description: gh_gist.description,
            files: gh_gist
                .files
                .into_values()
                .map(|f| GistFile {
                    filename: f.filename,
                    language: f.language,
                    size: f.size,
                })
                .collect(),
            updated_at: gh_gist.updated_at,
            public: gh_gist.public,
            html_url: gh_gist.html_url,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_datetime_serialization() {
        let metadata = CacheMetadata {
            last_updated: Utc::now(),
            total_count: 10,
            github_user: "testuser".to_string(),
        };

        let json = serde_json::to_string(&metadata).unwrap();
        assert!(json.contains("last_updated"));
        assert!(json.contains("total_count"));
        assert!(json.contains("github_user"));

        // Verify format matches bash script (no subseconds)
        assert!(!json.contains("."));
        assert!(json.contains("Z"));
    }

    #[test]
    fn test_datetime_deserialization() {
        let json = r#"{
            "last_updated": "2024-01-01T12:00:00Z",
            "total_count": 5,
            "github_user": "testuser"
        }"#;

        let metadata: CacheMetadata = serde_json::from_str(json).unwrap();
        assert_eq!(metadata.total_count, 5);
        assert_eq!(metadata.github_user, "testuser");
    }

    #[test]
    fn test_gist_cache_serialization() {
        let cache = GistCache {
            metadata: CacheMetadata {
                last_updated: Utc::now(),
                total_count: 1,
                github_user: "testuser".to_string(),
            },
            gists: vec![GistInfo {
                id: "abc123".to_string(),
                description: Some("Test gist".to_string()),
                files: vec![GistFile {
                    filename: "test.rs".to_string(),
                    language: Some("Rust".to_string()),
                    size: 100,
                }],
                updated_at: Utc::now(),
                public: true,
                html_url: "https://gist.github.com/abc123".to_string(),
            }],
        };

        let json = serde_json::to_string_pretty(&cache).unwrap();
        assert!(json.contains("metadata"));
        assert!(json.contains("gists"));
        assert!(json.contains("abc123"));
    }

    #[test]
    fn test_github_gist_to_gist_info() {
        let mut files = HashMap::new();
        files.insert(
            "test.rs".to_string(),
            GitHubFile {
                filename: "test.rs".to_string(),
                language: Some("Rust".to_string()),
                size: 100,
            },
        );

        let gh_gist = GitHubGist {
            id: "abc123".to_string(),
            description: Some("Test description".to_string()),
            files,
            updated_at: Utc::now(),
            public: true,
            html_url: "https://gist.github.com/abc123".to_string(),
        };

        let gist_info: GistInfo = gh_gist.into();
        assert_eq!(gist_info.id, "abc123");
        assert_eq!(gist_info.description, Some("Test description".to_string()));
        assert_eq!(gist_info.files.len(), 1);
        assert_eq!(gist_info.files[0].filename, "test.rs");
        assert_eq!(gist_info.files[0].language, Some("Rust".to_string()));
        assert_eq!(gist_info.files[0].size, 100);
        assert!(gist_info.public);
        assert_eq!(gist_info.html_url, "https://gist.github.com/abc123");
    }

    #[test]
    fn test_gist_file_clone() {
        let file = GistFile {
            filename: "test.rs".to_string(),
            language: Some("Rust".to_string()),
            size: 100,
        };

        let cloned = file.clone();
        assert_eq!(file.filename, cloned.filename);
        assert_eq!(file.language, cloned.language);
        assert_eq!(file.size, cloned.size);
    }

    #[test]
    fn test_gist_info_without_description() {
        let gist = GistInfo {
            id: "test123".to_string(),
            description: None,
            files: vec![],
            updated_at: Utc::now(),
            public: false,
            html_url: "https://gist.github.com/test123".to_string(),
        };

        assert_eq!(gist.description, None);
        assert!(!gist.public);
    }
}
