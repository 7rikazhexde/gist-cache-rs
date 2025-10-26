use crate::cache::types::GistInfo;
use crate::error::{GistCacheError, Result};

#[derive(Debug, Clone)]
pub enum SearchMode {
    Auto,
    Id,
    Filename,
    Description,
    Both,
}

pub struct SearchQuery {
    query: String,
    mode: SearchMode,
}

impl SearchQuery {
    pub fn new(query: String, mode: SearchMode) -> Self {
        Self { query, mode }
    }

    pub fn search<'a>(&self, gists: &'a [GistInfo]) -> Result<Vec<&'a GistInfo>> {
        let mode = match &self.mode {
            SearchMode::Auto => self.detect_mode(),
            other => other.clone(),
        };

        match mode {
            SearchMode::Id => self.search_by_id(gists),
            SearchMode::Filename => self.search_by_filename(gists),
            SearchMode::Description => self.search_by_description(gists),
            SearchMode::Both => self.search_both(gists),
            SearchMode::Auto => unreachable!(),
        }
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
}

pub fn select_from_results<'a>(results: &[&'a GistInfo]) -> Result<&'a GistInfo> {
    use std::io::{self, Write};

    if results.is_empty() {
        return Err(GistCacheError::NoSearchResults("".to_string()));
    }

    if results.len() == 1 {
        return Ok(results[0]);
    }

    println!("\n複数のGistが見つかりました:\n");

    let default_desc = "No description".to_string();

    for (i, gist) in results.iter().enumerate() {
        let desc = gist.description.as_ref().unwrap_or(&default_desc);
        let files: Vec<_> = gist.files.iter().map(|f| f.filename.as_str()).collect();
        println!(" {}. {} | {}", i + 1, desc, files.join(", "));
    }

    print!("\n番号を選択してください (1-{}): ", results.len());
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let selection: usize = input
        .trim()
        .parse()
        .map_err(|_| GistCacheError::InvalidSelection)?;

    if selection < 1 || selection > results.len() {
        return Err(GistCacheError::InvalidSelection);
    }

    Ok(results[selection - 1])
}
