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
#[derive(Debug, Deserialize)]
pub struct GitHubGist {
    pub id: String,
    pub description: Option<String>,
    pub files: std::collections::HashMap<String, GitHubFile>,
    pub updated_at: DateTime<Utc>,
    pub public: bool,
    pub html_url: String,
}

#[derive(Debug, Deserialize)]
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
