use crate::cache::types::{GitHubGist, RateLimit};
use crate::error::{GistCacheError, Result};
use chrono::{DateTime, Utc};
use std::process::Command;

pub struct GitHubApi;

impl GitHubApi {
    /// Check if GitHub CLI is authenticated
    pub fn check_auth() -> Result<()> {
        let output = Command::new("gh")
            .args(&["auth", "status"])
            .output()
            .map_err(|_| GistCacheError::NotAuthenticated)?;

        if !output.status.success() {
            return Err(GistCacheError::NotAuthenticated);
        }

        Ok(())
    }

    /// Get the authenticated GitHub user
    pub fn get_user() -> Result<String> {
        let output = Command::new("gh")
            .args(&["api", "user", "--jq", ".login"])
            .output()?;

        if !output.status.success() {
            return Err(GistCacheError::GitHubApi(
                "Failed to get user information".to_string(),
            ));
        }

        let user = String::from_utf8_lossy(&output.stdout).trim().to_string();
        Ok(user)
    }

    /// Check rate limit
    pub fn check_rate_limit() -> Result<i64> {
        let output = Command::new("gh").args(&["api", "rate_limit"]).output()?;

        if !output.status.success() {
            return Err(GistCacheError::GitHubApi(
                "Failed to check rate limit".to_string(),
            ));
        }

        let rate_limit: RateLimit = serde_json::from_slice(&output.stdout)?;
        Ok(rate_limit.resources.core.remaining)
    }

    /// Fetch gists with optional since parameter for differential updates
    pub fn fetch_gists(since: Option<DateTime<Utc>>) -> Result<Vec<GitHubGist>> {
        let mut args = vec![
            "api".to_string(),
            "/gists?per_page=100".to_string(),
            "--paginate".to_string(),
        ];

        if let Some(since_date) = since {
            // Format as ISO 8601 without subseconds to match bash script
            let since_str = since_date.format("%Y-%m-%dT%H:%M:%SZ").to_string();
            args[1] = format!("/gists?since={}&per_page=100", since_str);
        }

        let output = Command::new("gh").args(&args).output()?;

        if !output.status.success() {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            return Err(GistCacheError::GitHubApi(format!(
                "Failed to fetch gists: {}",
                error_msg
            )));
        }

        // Parse the paginated JSON response
        let gists_str = String::from_utf8_lossy(&output.stdout);
        let mut all_gists = Vec::new();

        // GitHub CLI returns paginated results as multiple JSON arrays
        for line in gists_str.lines() {
            if line.trim().is_empty() {
                continue;
            }

            match serde_json::from_str::<Vec<GitHubGist>>(line) {
                Ok(mut gists) => {
                    all_gists.append(&mut gists);
                }
                Err(e) => {
                    eprintln!("Warning: Failed to parse line: {} - {}", line, e);
                }
            }
        }

        Ok(all_gists)
    }

    /// Fetch a single gist by ID
    pub fn fetch_gist_content(gist_id: &str, filename: &str) -> Result<String> {
        let output = Command::new("gh")
            .args(&["gist", "view", gist_id, "--filename", filename, "--raw"])
            .output()?;

        if !output.status.success() {
            return Err(GistCacheError::GitHubApi(format!(
                "Failed to fetch gist content: {}",
                gist_id
            )));
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}
