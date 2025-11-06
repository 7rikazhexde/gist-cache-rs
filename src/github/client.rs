use crate::cache::types::GitHubGist;
use crate::error::Result;
use chrono::{DateTime, Utc};

/// GitHub API操作のトレイト
/// テスト時にモック実装を注入できるようにする
#[cfg_attr(test, mockall::automock)]
pub trait GitHubClient {
    /// GitHub CLIの認証状態を確認
    fn check_auth(&self) -> Result<()>;

    /// 認証済みユーザー名を取得
    fn get_user(&self) -> Result<String>;

    /// APIレート制限の残数を確認
    fn check_rate_limit(&self) -> Result<i64>;

    /// Gist一覧を取得（差分更新用のsinceパラメータ対応）
    fn fetch_gists(&self, since: Option<DateTime<Utc>>) -> Result<Vec<GitHubGist>>;

    /// 特定Gistファイルの内容を取得
    fn fetch_gist_content(&self, gist_id: &str, filename: &str) -> Result<String>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_mock_check_auth_success() {
        let mut mock = MockGitHubClient::new();
        mock.expect_check_auth().times(1).returning(|| Ok(()));

        assert!(mock.check_auth().is_ok());
    }

    #[test]
    fn test_mock_get_user() {
        let mut mock = MockGitHubClient::new();
        mock.expect_get_user()
            .times(1)
            .returning(|| Ok("testuser".to_string()));

        let result = mock.get_user().unwrap();
        assert_eq!(result, "testuser");
    }

    #[test]
    fn test_mock_check_rate_limit() {
        let mut mock = MockGitHubClient::new();
        mock.expect_check_rate_limit()
            .times(1)
            .returning(|| Ok(5000));

        let remaining = mock.check_rate_limit().unwrap();
        assert_eq!(remaining, 5000);
    }

    #[test]
    fn test_mock_fetch_gists() {
        let mut mock = MockGitHubClient::new();

        // テスト用のGistデータを作成
        let test_gist = GitHubGist {
            id: "test123".to_string(),
            description: Some("Test gist".to_string()),
            files: HashMap::from([(
                "test.rs".to_string(),
                crate::cache::types::GitHubFile {
                    filename: "test.rs".to_string(),
                    language: Some("Rust".to_string()),
                    size: 100,
                },
            )]),
            updated_at: Utc::now(),
            public: true,
            html_url: "https://gist.github.com/test123".to_string(),
        };

        mock.expect_fetch_gists()
            .times(1)
            .returning(move |_| Ok(vec![test_gist.clone()]));

        let gists = mock.fetch_gists(None).unwrap();
        assert_eq!(gists.len(), 1);
        assert_eq!(gists[0].id, "test123");
    }

    #[test]
    fn test_mock_fetch_gist_content() {
        let mut mock = MockGitHubClient::new();
        mock.expect_fetch_gist_content()
            .with(
                mockall::predicate::eq("test123"),
                mockall::predicate::eq("test.rs"),
            )
            .times(1)
            .returning(|_, _| Ok("# Test content".to_string()));

        let content = mock.fetch_gist_content("test123", "test.rs").unwrap();
        assert_eq!(content, "# Test content");
    }
}
