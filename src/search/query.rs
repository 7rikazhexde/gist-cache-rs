use crate::cache::types::GistInfo;
use crate::error::{GistCacheError, Result};
use regex::Regex;

#[derive(Debug, Clone)]
pub enum SearchMode {
    Auto,
    Id,
    Filename,
    Description,
    Both,
}

#[derive(Debug, Clone, Default)]
pub struct SearchOptions {
    pub regex: Option<String>,
    pub language: Option<String>,
    pub extension: Option<String>,
}

pub struct SearchQuery {
    query: String,
    mode: SearchMode,
    options: SearchOptions,
}

impl SearchQuery {
    pub fn new(query: String, mode: SearchMode) -> Self {
        Self {
            query,
            mode,
            options: SearchOptions::default(),
        }
    }

    pub fn new_with_options(query: String, mode: SearchMode, options: SearchOptions) -> Self {
        Self {
            query,
            mode,
            options,
        }
    }

    pub fn search<'a>(&self, gists: &'a [GistInfo]) -> Result<Vec<&'a GistInfo>> {
        let mode = match &self.mode {
            SearchMode::Auto => self.detect_mode(),
            other => other.clone(),
        };

        let mut results = match mode {
            SearchMode::Id => self.search_by_id(gists),
            SearchMode::Filename => self.search_by_filename(gists),
            SearchMode::Description => self.search_by_description(gists),
            SearchMode::Both => self.search_both(gists),
            SearchMode::Auto => unreachable!(),
        }?;

        // Apply additional filters
        results = self.apply_filters(results)?;

        Ok(results)
    }

    fn detect_mode(&self) -> SearchMode {
        if self.query.len() == 32 && self.query.chars().all(|c| c.is_ascii_hexdigit()) {
            SearchMode::Id
        } else {
            SearchMode::Both
        }
    }

    fn search_by_id<'a>(&self, gists: &'a [GistInfo]) -> Result<Vec<&'a GistInfo>> {
        let result = gists.iter().filter(|g| g.id == self.query).collect();
        Ok(result)
    }

    fn search_by_filename<'a>(&self, gists: &'a [GistInfo]) -> Result<Vec<&'a GistInfo>> {
        let results: Vec<&GistInfo> = gists
            .iter()
            .filter(|g| {
                g.files.iter().any(|f| {
                    f.filename
                        .to_lowercase()
                        .contains(&self.query.to_lowercase())
                })
            })
            .collect();
        Ok(results)
    }

    fn search_by_description<'a>(&self, gists: &'a [GistInfo]) -> Result<Vec<&'a GistInfo>> {
        let results: Vec<&GistInfo> = gists
            .iter()
            .filter(|g| {
                g.description
                    .as_ref()
                    .map(|d| d.to_lowercase().contains(&self.query.to_lowercase()))
                    .unwrap_or(false)
            })
            .collect();
        Ok(results)
    }

    fn search_both<'a>(&self, gists: &'a [GistInfo]) -> Result<Vec<&'a GistInfo>> {
        let query_lower = self.query.to_lowercase();
        let results: Vec<&GistInfo> = gists
            .iter()
            .filter(|g| {
                let desc_match = g
                    .description
                    .as_ref()
                    .map(|d| d.to_lowercase().contains(&query_lower))
                    .unwrap_or(false);

                let file_match = g
                    .files
                    .iter()
                    .any(|f| f.filename.to_lowercase().contains(&query_lower));

                desc_match || file_match
            })
            .collect();
        Ok(results)
    }

    fn apply_filters<'a>(&self, results: Vec<&'a GistInfo>) -> Result<Vec<&'a GistInfo>> {
        let mut filtered = results;

        // Apply regex filter
        if let Some(pattern) = &self.options.regex {
            filtered = self.filter_by_regex(filtered, pattern)?;
        }

        // Apply language filter
        if let Some(language) = &self.options.language {
            filtered = self.filter_by_language(filtered, language);
        }

        // Apply extension filter
        if let Some(extension) = &self.options.extension {
            filtered = self.filter_by_extension(filtered, extension);
        }

        Ok(filtered)
    }

    fn filter_by_regex<'a>(
        &self,
        results: Vec<&'a GistInfo>,
        pattern: &str,
    ) -> Result<Vec<&'a GistInfo>> {
        let re = Regex::new(pattern).map_err(|e| GistCacheError::InvalidPattern(e.to_string()))?;

        let filtered = results
            .into_iter()
            .filter(|g| {
                // Check description
                let desc_match = g
                    .description
                    .as_ref()
                    .map(|d| re.is_match(d))
                    .unwrap_or(false);

                // Check filenames
                let file_match = g.files.iter().any(|f| re.is_match(&f.filename));

                desc_match || file_match
            })
            .collect();

        Ok(filtered)
    }

    fn filter_by_language<'a>(
        &self,
        results: Vec<&'a GistInfo>,
        language: &str,
    ) -> Vec<&'a GistInfo> {
        let language_lower = language.to_lowercase();
        results
            .into_iter()
            .filter(|g| {
                g.files.iter().any(|f| {
                    f.language
                        .as_ref()
                        .map(|l| l.to_lowercase() == language_lower)
                        .unwrap_or(false)
                })
            })
            .collect()
    }

    fn filter_by_extension<'a>(
        &self,
        results: Vec<&'a GistInfo>,
        extension: &str,
    ) -> Vec<&'a GistInfo> {
        let ext = if extension.starts_with('.') {
            extension.to_string()
        } else {
            format!(".{}", extension)
        };
        let ext_lower = ext.to_lowercase();

        results
            .into_iter()
            .filter(|g| {
                g.files
                    .iter()
                    .any(|f| f.filename.to_lowercase().ends_with(&ext_lower))
            })
            .collect()
    }
}

pub fn select_from_results<'a>(results: &[&'a GistInfo]) -> Result<&'a GistInfo> {
    use dialoguer::{Select, theme::ColorfulTheme};

    if results.is_empty() {
        return Err(GistCacheError::NoSearchResults("".to_string()));
    }

    if results.len() == 1 {
        return Ok(results[0]);
    }

    let default_desc = "No description".to_string();

    // Create display items: "description - files"
    let items: Vec<String> = results
        .iter()
        .map(|gist| {
            let desc = gist.description.as_ref().unwrap_or(&default_desc);
            let files: Vec<_> = gist.files.iter().map(|f| f.filename.as_str()).collect();
            format!("{} - {}", desc, files.join(", "))
        })
        .collect();

    println!("\nMultiple Gists found:\n");

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a Gist")
        .items(&items)
        .default(0)
        .interact_opt()
        .map_err(|e| GistCacheError::Io(std::io::Error::other(e)))?;

    match selection {
        Some(index) => Ok(results[index]),
        None => Err(GistCacheError::InvalidSelection),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    fn create_test_gist(id: &str, desc: Option<&str>, filenames: Vec<&str>) -> GistInfo {
        GistInfo {
            id: id.to_string(),
            description: desc.map(|s| s.to_string()),
            files: filenames
                .into_iter()
                .map(|name| crate::cache::types::GistFile {
                    filename: name.to_string(),
                    language: None,
                    size: 100,
                })
                .collect(),
            updated_at: Utc::now(),
            public: true,
            html_url: format!("https://gist.github.com/{}", id),
        }
    }

    #[test]
    fn test_search_mode_auto_detects_id() {
        let query = SearchQuery::new("a".repeat(32), SearchMode::Auto);
        // 32-char hex string should be detected as ID
        let mode = query.detect_mode();
        assert!(matches!(mode, SearchMode::Id));
    }

    #[test]
    fn test_search_mode_auto_detects_both() {
        let query = SearchQuery::new("test".to_string(), SearchMode::Auto);
        let mode = query.detect_mode();
        assert!(matches!(mode, SearchMode::Both));
    }

    #[test]
    fn test_search_by_id() {
        let gists = vec![
            create_test_gist("abc123", Some("Test 1"), vec!["file1.rs"]),
            create_test_gist("def456", Some("Test 2"), vec!["file2.rs"]),
        ];

        let query = SearchQuery::new("abc123".to_string(), SearchMode::Id);
        let results = query.search(&gists).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, "abc123");
    }

    #[test]
    fn test_search_by_filename() {
        let gists = vec![
            create_test_gist("abc123", Some("Test 1"), vec!["hello.rs"]),
            create_test_gist("def456", Some("Test 2"), vec!["world.js"]),
        ];

        let query = SearchQuery::new("hello".to_string(), SearchMode::Filename);
        let results = query.search(&gists).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, "abc123");
    }

    #[test]
    fn test_search_by_filename_case_insensitive() {
        let gists = vec![create_test_gist("abc123", Some("Test"), vec!["Hello.rs"])];

        let query = SearchQuery::new("hello".to_string(), SearchMode::Filename);
        let results = query.search(&gists).unwrap();
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_search_by_description() {
        let gists = vec![
            create_test_gist("abc123", Some("Rust script"), vec!["file.rs"]),
            create_test_gist("def456", Some("Python script"), vec!["file.py"]),
        ];

        let query = SearchQuery::new("rust".to_string(), SearchMode::Description);
        let results = query.search(&gists).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, "abc123");
    }

    #[test]
    fn test_search_by_description_case_insensitive() {
        let gists = vec![create_test_gist(
            "abc123",
            Some("Rust Script"),
            vec!["file.rs"],
        )];

        let query = SearchQuery::new("rust".to_string(), SearchMode::Description);
        let results = query.search(&gists).unwrap();
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_search_by_description_no_description() {
        let gists = vec![create_test_gist("abc123", None, vec!["file.rs"])];

        let query = SearchQuery::new("rust".to_string(), SearchMode::Description);
        let results = query.search(&gists).unwrap();
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_search_both() {
        let gists = vec![
            create_test_gist("abc123", Some("Rust script"), vec!["main.rs"]),
            create_test_gist("def456", Some("Python script"), vec!["rust_file.py"]),
            create_test_gist("ghi789", Some("Other"), vec!["file.js"]),
        ];

        let query = SearchQuery::new("rust".to_string(), SearchMode::Both);
        let results = query.search(&gists).unwrap();
        assert_eq!(results.len(), 2); // Matches both description and filename
    }

    #[test]
    fn test_search_no_results() {
        let gists = vec![create_test_gist("abc123", Some("Test"), vec!["file.rs"])];

        let query = SearchQuery::new("nonexistent".to_string(), SearchMode::Both);
        let results = query.search(&gists).unwrap();
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_select_from_empty_results() {
        let results: Vec<&GistInfo> = vec![];
        let error = select_from_results(&results).unwrap_err();
        assert!(matches!(error, GistCacheError::NoSearchResults(_)));
    }

    #[test]
    fn test_select_from_single_result() {
        let gist = create_test_gist("abc123", Some("Test"), vec!["file.rs"]);
        let results = vec![&gist];
        let selected = select_from_results(&results).unwrap();
        assert_eq!(selected.id, "abc123");
    }

    #[test]
    fn test_search_with_auto_mode_id() {
        let gists = vec![
            create_test_gist(
                "abc123def456789012345678901234ab",
                Some("Test 1"),
                vec!["file1.rs"],
            ),
            create_test_gist("def456", Some("Test 2"), vec!["file2.rs"]),
        ];

        // 32-char hex string should trigger ID mode
        let query = SearchQuery::new(
            "abc123def456789012345678901234ab".to_string(),
            SearchMode::Auto,
        );
        let results = query.search(&gists).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, "abc123def456789012345678901234ab");
    }

    #[test]
    fn test_search_with_auto_mode_keyword() {
        let gists = vec![
            create_test_gist("abc123", Some("Rust script"), vec!["main.rs"]),
            create_test_gist("def456", Some("Python script"), vec!["main.py"]),
        ];

        // Non-ID query should trigger Both mode
        let query = SearchQuery::new("rust".to_string(), SearchMode::Auto);
        let results = query.search(&gists).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, "abc123");
    }

    #[test]
    fn test_search_both_description_only_match() {
        let gists = vec![
            create_test_gist("abc123", Some("contains keyword"), vec!["other.rs"]),
            create_test_gist("def456", Some("no match"), vec!["file.py"]),
        ];

        let query = SearchQuery::new("keyword".to_string(), SearchMode::Both);
        let results = query.search(&gists).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, "abc123");
    }

    #[test]
    fn test_search_both_filename_only_match() {
        let gists = vec![
            create_test_gist("abc123", Some("description"), vec!["keyword.rs"]),
            create_test_gist("def456", Some("other"), vec!["file.py"]),
        ];

        let query = SearchQuery::new("keyword".to_string(), SearchMode::Both);
        let results = query.search(&gists).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, "abc123");
    }

    #[test]
    fn test_search_both_no_description() {
        let gists = vec![
            create_test_gist("abc123", None, vec!["keyword.rs"]),
            create_test_gist("def456", Some("has desc"), vec!["file.py"]),
        ];

        let query = SearchQuery::new("keyword".to_string(), SearchMode::Both);
        let results = query.search(&gists).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, "abc123");
    }

    #[test]
    fn test_search_filename_multiple_files() {
        let gists = vec![
            create_test_gist("abc123", Some("Test"), vec!["main.rs", "lib.rs"]),
            create_test_gist("def456", Some("Test"), vec!["test.py", "target.rs"]),
        ];

        let query = SearchQuery::new("target".to_string(), SearchMode::Filename);
        let results = query.search(&gists).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, "def456");
    }

    #[test]
    fn test_search_both_multiple_matches() {
        let gists = vec![
            create_test_gist("abc123", Some("rust programming"), vec!["main.rs"]),
            create_test_gist("def456", Some("python code"), vec!["rust.py"]),
            create_test_gist("ghi789", Some("javascript"), vec!["app.js"]),
        ];

        let query = SearchQuery::new("rust".to_string(), SearchMode::Both);
        let results = query.search(&gists).unwrap();
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_search_empty_gist_list() {
        let gists: Vec<GistInfo> = vec![];

        let query = SearchQuery::new("test".to_string(), SearchMode::Both);
        let results = query.search(&gists).unwrap();
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_search_id_no_match() {
        let gists = vec![create_test_gist("abc123", Some("Test"), vec!["file.rs"])];

        let query = SearchQuery::new("xyz999".to_string(), SearchMode::Id);
        let results = query.search(&gists).unwrap();
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_search_mode_auto_with_short_hex() {
        // 31-char hex string should not be detected as ID (needs 32)
        let query = SearchQuery::new("a".repeat(31), SearchMode::Auto);
        let mode = query.detect_mode();
        assert!(matches!(mode, SearchMode::Both));
    }

    #[test]
    fn test_search_mode_auto_with_non_hex() {
        // 32-char but not all hex should not be detected as ID
        let query = SearchQuery::new("g".repeat(32), SearchMode::Auto);
        let mode = query.detect_mode();
        assert!(matches!(mode, SearchMode::Both));
    }

    // Helper function to create test gist with language
    fn create_test_gist_with_language(
        id: &str,
        desc: Option<&str>,
        files: Vec<(&str, Option<&str>)>,
    ) -> GistInfo {
        GistInfo {
            id: id.to_string(),
            description: desc.map(|s| s.to_string()),
            files: files
                .into_iter()
                .map(|(name, lang)| crate::cache::types::GistFile {
                    filename: name.to_string(),
                    language: lang.map(|l| l.to_string()),
                    size: 100,
                })
                .collect(),
            updated_at: Utc::now(),
            public: true,
            html_url: format!("https://gist.github.com/{}", id),
        }
    }

    #[test]
    fn test_filter_by_regex() {
        let gists = vec![
            create_test_gist("abc123", Some("Test regex pattern"), vec!["file1.rs"]),
            create_test_gist("def456", Some("Another test"), vec!["file2.py"]),
            create_test_gist("ghi789", Some("No match"), vec!["file3.js"]),
        ];

        let options = SearchOptions {
            regex: Some(r"^Test".to_string()),
            language: None,
            extension: None,
        };

        let query = SearchQuery::new_with_options("test".to_string(), SearchMode::Both, options);
        let results = query.search(&gists).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, "abc123");
    }

    #[test]
    fn test_filter_by_regex_filename() {
        let gists = vec![
            create_test_gist("abc123", Some("Description"), vec!["config.toml"]),
            create_test_gist("def456", Some("Description"), vec!["script.py"]),
            create_test_gist("ghi789", Some("Description"), vec!["main.rs"]),
        ];

        let options = SearchOptions {
            regex: Some(r"\.toml$".to_string()),
            language: None,
            extension: None,
        };

        let query = SearchQuery::new_with_options("".to_string(), SearchMode::Both, options);
        let results = query.search(&gists).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, "abc123");
    }

    #[test]
    fn test_filter_by_invalid_regex() {
        let gists = vec![create_test_gist("abc123", Some("Test"), vec!["file.rs"])];

        let options = SearchOptions {
            regex: Some("[invalid".to_string()),
            language: None,
            extension: None,
        };

        let query = SearchQuery::new_with_options("test".to_string(), SearchMode::Both, options);
        let result = query.search(&gists);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            GistCacheError::InvalidPattern(_)
        ));
    }

    #[test]
    fn test_filter_by_language() {
        let gists = vec![
            create_test_gist_with_language(
                "abc123",
                Some("Rust script"),
                vec![("main.rs", Some("Rust"))],
            ),
            create_test_gist_with_language(
                "def456",
                Some("Python script"),
                vec![("script.py", Some("Python"))],
            ),
            create_test_gist_with_language(
                "ghi789",
                Some("JS file"),
                vec![("app.js", Some("JavaScript"))],
            ),
        ];

        let options = SearchOptions {
            regex: None,
            language: Some("rust".to_string()),
            extension: None,
        };

        let query = SearchQuery::new_with_options("script".to_string(), SearchMode::Both, options);
        let results = query.search(&gists).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, "abc123");
    }

    #[test]
    fn test_filter_by_language_case_insensitive() {
        let gists = vec![
            create_test_gist_with_language(
                "abc123",
                Some("Test"),
                vec![("file.py", Some("Python"))],
            ),
            create_test_gist_with_language("def456", Some("Test"), vec![("file.rs", Some("Rust"))]),
        ];

        let options = SearchOptions {
            regex: None,
            language: Some("PYTHON".to_string()),
            extension: None,
        };

        let query = SearchQuery::new_with_options("test".to_string(), SearchMode::Both, options);
        let results = query.search(&gists).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, "abc123");
    }

    #[test]
    fn test_filter_by_language_no_match() {
        let gists = vec![create_test_gist_with_language(
            "abc123",
            Some("Test"),
            vec![("file.py", Some("Python"))],
        )];

        let options = SearchOptions {
            regex: None,
            language: Some("Rust".to_string()),
            extension: None,
        };

        let query = SearchQuery::new_with_options("test".to_string(), SearchMode::Both, options);
        let results = query.search(&gists).unwrap();
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_filter_by_extension() {
        let gists = vec![
            create_test_gist("abc123", Some("Rust file"), vec!["main.rs"]),
            create_test_gist("def456", Some("Python file"), vec!["script.py"]),
            create_test_gist("ghi789", Some("Config file"), vec!["config.toml"]),
        ];

        let options = SearchOptions {
            regex: None,
            language: None,
            extension: Some("rs".to_string()),
        };

        let query = SearchQuery::new_with_options("file".to_string(), SearchMode::Both, options);
        let results = query.search(&gists).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, "abc123");
    }

    #[test]
    fn test_filter_by_extension_with_dot() {
        let gists = vec![
            create_test_gist("abc123", Some("Test"), vec!["file.py"]),
            create_test_gist("def456", Some("Test"), vec!["file.rs"]),
        ];

        let options = SearchOptions {
            regex: None,
            language: None,
            extension: Some(".py".to_string()),
        };

        let query = SearchQuery::new_with_options("test".to_string(), SearchMode::Both, options);
        let results = query.search(&gists).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, "abc123");
    }

    #[test]
    fn test_filter_by_extension_case_insensitive() {
        let gists = vec![
            create_test_gist("abc123", Some("Test"), vec!["File.RS"]),
            create_test_gist("def456", Some("Test"), vec!["file.py"]),
        ];

        let options = SearchOptions {
            regex: None,
            language: None,
            extension: Some("RS".to_string()),
        };

        let query = SearchQuery::new_with_options("test".to_string(), SearchMode::Both, options);
        let results = query.search(&gists).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, "abc123");
    }

    #[test]
    fn test_combined_filters() {
        let gists = vec![
            create_test_gist_with_language(
                "abc123",
                Some("Rust config"),
                vec![("config.toml", Some("TOML"))],
            ),
            create_test_gist_with_language(
                "def456",
                Some("Rust script"),
                vec![("main.rs", Some("Rust"))],
            ),
            create_test_gist_with_language(
                "ghi789",
                Some("Python script"),
                vec![("script.py", Some("Python"))],
            ),
        ];

        let options = SearchOptions {
            regex: Some(r"^Rust".to_string()),
            language: Some("Rust".to_string()),
            extension: Some("rs".to_string()),
        };

        let query = SearchQuery::new_with_options("script".to_string(), SearchMode::Both, options);
        let results = query.search(&gists).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, "def456");
    }

    #[test]
    fn test_filter_with_multiple_files() {
        let gists = vec![
            create_test_gist_with_language(
                "abc123",
                Some("Mixed project"),
                vec![("main.rs", Some("Rust")), ("script.py", Some("Python"))],
            ),
            create_test_gist_with_language(
                "def456",
                Some("Rust only"),
                vec![("lib.rs", Some("Rust"))],
            ),
        ];

        let options = SearchOptions {
            regex: None,
            language: Some("Python".to_string()),
            extension: None,
        };

        let query = SearchQuery::new_with_options("project".to_string(), SearchMode::Both, options);
        let results = query.search(&gists).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, "abc123");
    }

    #[test]
    fn test_no_filters_applied() {
        let gists = vec![
            create_test_gist("abc123", Some("Test 1"), vec!["file1.rs"]),
            create_test_gist("def456", Some("Test 2"), vec!["file2.py"]),
        ];

        let options = SearchOptions::default();

        let query = SearchQuery::new_with_options("test".to_string(), SearchMode::Both, options);
        let results = query.search(&gists).unwrap();
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_filter_results_in_no_matches() {
        let gists = vec![create_test_gist_with_language(
            "abc123",
            Some("Python script"),
            vec![("script.py", Some("Python"))],
        )];

        let options = SearchOptions {
            regex: None,
            language: Some("Rust".to_string()),
            extension: None,
        };

        let query = SearchQuery::new_with_options("script".to_string(), SearchMode::Both, options);
        let results = query.search(&gists).unwrap();
        assert_eq!(results.len(), 0);
    }
}
