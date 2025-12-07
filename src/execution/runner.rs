use crate::cache::ContentCache;
use crate::cache::types::GistInfo;
use crate::config::Config;
use crate::error::{GistCacheError, Result};
use crate::github::GitHubApi;
use colored::Colorize;
use std::fs;
use std::io::Write;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::process::{Command, Stdio};

/// Options for script execution
pub struct RunOptions {
    pub interactive: bool,
    pub preview: bool,
    pub download: bool,
    pub force_file_based: bool,
}

pub struct ScriptRunner {
    gist: GistInfo,
    interpreter: String,
    run_command: Option<String>,
    is_shell: bool,
    options: RunOptions,
    args: Vec<String>,
    config: Config,
}

impl ScriptRunner {
    pub fn new(
        gist: GistInfo,
        interpreter: String,
        run_command: Option<String>,
        is_shell: bool,
        options: RunOptions,
        args: Vec<String>,
        config: Config,
    ) -> Self {
        Self {
            gist,
            interpreter,
            run_command,
            is_shell,
            options,
            args,
            config,
        }
    }

    pub fn run(&self) -> Result<()> {
        // Display gist info
        self.display_info();

        if self.options.preview {
            self.preview_content()?;
            if self.options.download {
                return self.download_files();
            }
            return Ok(());
        }

        let result = self.execute();

        // If download option is specified, download regardless of execution result
        if self.options.download {
            self.download_files()?;
        }

        result
    }

    fn display_info(&self) {
        println!();
        println!(
            "{}",
            format!(
                "Description: {}",
                self.gist
                    .description
                    .as_ref()
                    .unwrap_or(&"No description".to_string())
            )
            .cyan()
        );
        print!("Files: ");
        for (i, file) in self.gist.files.iter().enumerate() {
            if i > 0 {
                print!(", ");
            }
            print!("{}", file.filename.green());
        }
        println!("\n");
    }

    fn preview_content(&self) -> Result<()> {
        println!("{}", "=== Gist Content ===".cyan().bold());

        for file in &self.gist.files {
            println!("\n{}", format!("--- {} ---", file.filename).yellow().bold());

            // Check cache
            let content_cache = ContentCache::new(self.config.contents_dir.clone());

            let content = if content_cache.exists(&self.gist.id, &file.filename) {
                // Load from cache
                match content_cache.read(&self.gist.id, &file.filename) {
                    Ok(c) => c,
                    Err(_) => {
                        // Fetch from API if cache read fails
                        GitHubApi::new().fetch_gist_content(&self.gist.id, &file.filename)?
                    }
                }
            } else {
                // Fetch from API
                GitHubApi::new().fetch_gist_content(&self.gist.id, &file.filename)?
            };

            println!("{}", content);
        }

        Ok(())
    }

    fn execute(&self) -> Result<()> {
        // Select the main file to execute
        let main_file = self.select_main_file()?;

        println!(
            "{}",
            format!("Executing: {} ({})", main_file.filename, self.interpreter).cyan()
        );

        // Check cache and fetch content
        let content_cache = ContentCache::new(self.config.contents_dir.clone());

        let content = if content_cache.exists(&self.gist.id, &main_file.filename) {
            // Load from cache
            match content_cache.read(&self.gist.id, &main_file.filename) {
                Ok(c) => {
                    if std::env::var("GIST_CACHE_VERBOSE").is_ok() {
                        println!("{}", "  → Loaded from cache".green());
                    }
                    c
                }
                Err(e) => {
                    // Self-healing principle: Fetch from API if cache read fails
                    eprintln!(
                        "{}",
                        format!("  Warning: Cache read failed, fetching from API: {}", e)
                            .yellow()
                    );
                    let fetched =
                        GitHubApi::new().fetch_gist_content(&self.gist.id, &main_file.filename)?;

                    // Try to save to cache if fetch succeeds
                    let _ = content_cache.write(&self.gist.id, &main_file.filename, &fetched);

                    fetched
                }
            }
        } else {
            // Fetch from API
            println!(
                "{}",
                "  Info: Cache does not exist, fetching from GitHub API...".yellow()
            );
            GitHubApi::new().fetch_gist_content(&self.gist.id, &main_file.filename)?
        };

        // Ensure consistency in interactive mode:
        // Whether loading from cache or fetching from API,
        // always execute via temporary file to unify behavior
        let execution_result =
            if self.options.force_file_based || self.options.interactive || self.is_shell {
                self.execute_via_temp_file(&content, &main_file.filename)
            } else {
                self.execute_direct(&content)
            };

        // Save to cache only if execution succeeds
        if execution_result.is_ok() {
            // Save only if cache doesn't exist (don't overwrite existing cache)
            if !content_cache.exists(&self.gist.id, &main_file.filename) {
                match content_cache.write(&self.gist.id, &main_file.filename, &content) {
                    Ok(_) => {
                        if std::env::var("GIST_CACHE_VERBOSE").is_ok() {
                            println!(
                                "{}",
                                format!(
                                    "  → Saved to cache: {}",
                                    self.config
                                        .contents_dir
                                        .join(&self.gist.id)
                                        .join(&main_file.filename)
                                        .display()
                                )
                                .green()
                            );
                        }
                    }
                    Err(e) => {
                        // Cache save failure is warning only (execution succeeded)
                        eprintln!(
                            "{}",
                            format!("  Warning: Failed to save cache: {}", e).yellow()
                        );
                    }
                }
            }
        }

        execution_result
    }

    fn select_main_file(&self) -> Result<&crate::cache::types::GistFile> {
        if self.gist.files.len() == 1 {
            return Ok(&self.gist.files[0]);
        }

        // Try to find a matching file based on interpreter
        let preferred_extension = match self.interpreter.as_str() {
            "bash" | "sh" => ".sh",
            "python" | "python3" => ".py",
            "ruby" => ".rb",
            "node" => ".js",
            "perl" => ".pl",
            "php" => ".php",
            "pwsh" | "powershell" => ".ps1",
            "ts-node" | "deno" | "bun" => ".ts",
            _ => "",
        };

        if !preferred_extension.is_empty()
            && let Some(file) = self
                .gist
                .files
                .iter()
                .find(|f| f.filename.ends_with(preferred_extension))
        {
            return Ok(file);
        }

        // Default to first file
        Ok(&self.gist.files[0])
    }

    /// Execute via temporary file (interactive mode, shell scripts, file-based interpreters)
    ///
    /// Important: Using this function ensures consistent behavior for
    /// interactive scripts regardless of whether cache exists or not
    fn execute_via_temp_file(&self, content: &str, filename: &str) -> Result<()> {
        // Create temporary file
        let temp_dir = std::env::temp_dir();
        let temp_file = temp_dir.join(filename);

        fs::write(&temp_file, content)?;

        // Make executable for shell scripts (Unix only)
        #[cfg(unix)]
        if self.is_shell {
            let mut perms = fs::metadata(&temp_file)?.permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&temp_file, perms)?;
        }

        // Windows: No need to set executable permission
        // File extension (.bat, .ps1, etc.) determines executability

        // Build command
        let mut cmd = if let Some(ref run_cmd) = self.run_command {
            let parts: Vec<&str> = run_cmd.split_whitespace().collect();
            let mut command = Command::new(parts[0]);
            for part in &parts[1..] {
                command.arg(part);
            }
            command.arg(&temp_file);
            command
        } else if self.is_shell {
            Command::new(&temp_file)
        } else {
            let mut command = Command::new(&self.interpreter);

            // Add compiler options for ts-node (Node.js v22+ ESM compatibility)
            if self.interpreter == "ts-node" {
                command.arg("--compilerOptions");
                command.arg(r#"{"module":"commonjs","moduleResolution":"node"}"#);
            }

            command.arg(&temp_file);
            command
        };

        // Add user arguments
        for arg in &self.args {
            cmd.arg(arg);
        }

        // Run with inherited stdio for interactive mode
        // Inherit stdin in interactive mode so commands like `read` work properly
        let status = cmd
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()?;

        // Clean up
        let _ = fs::remove_file(&temp_file);

        if !status.success() {
            return Err(GistCacheError::Execution(format!(
                "Script exited with code: {}",
                status.code().unwrap_or(-1)
            )));
        }

        Ok(())
    }

    /// Direct execution via stdin (non-interactive mode, stdin-compatible interpreters)
    fn execute_direct(&self, content: &str) -> Result<()> {
        // Build command with interpreter-specific flags for stdin execution
        let mut cmd = match self.interpreter.as_str() {
            "python" | "python3" => {
                // Python: Use '-' flag for stdin execution (required)
                let mut command = Command::new(&self.interpreter);
                command.arg("-");
                command
            }
            "ruby" => {
                // Ruby: Use '-' flag for stdin execution
                let mut command = Command::new(&self.interpreter);
                command.arg("-");
                command
            }
            "node" => {
                // Node.js: Use '-' flag for stdin execution
                let mut command = Command::new(&self.interpreter);
                command.arg("-");
                command
            }
            "perl" => {
                // Perl: Use '-' flag for stdin execution
                let mut command = Command::new(&self.interpreter);
                command.arg("-");
                command
            }
            _ => {
                // Fallback: Assume file-based or warn (PHP is now file-based via parse_interpreter)
                return Err(GistCacheError::Execution(format!(
                    "Direct execution not supported for interpreter '{}'. Use file-based mode.",
                    self.interpreter
                )));
            }
        };

        if let Some(ref run_cmd) = self.run_command {
            // Handle custom run_command (e.g., uv) - adjust as needed
            let parts: Vec<&str> = run_cmd.split_whitespace().collect();
            let mut command = Command::new(parts[0]);
            for part in &parts[1..] {
                command.arg(part);
            }
            // For uv, append stdin handling if necessary (uv run typically file-based)
            cmd = command;
        }

        // Add user arguments (after flags)
        for arg in &self.args {
            cmd.arg(arg);
        }

        // Execute with piped stdin
        let mut child = cmd
            .stdin(Stdio::piped())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()?;

        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(content.as_bytes())?;
        }

        let status = child.wait()?;

        if !status.success() {
            return Err(GistCacheError::Execution(format!(
                "Script exited with code: {}",
                status.code().unwrap_or(-1)
            )));
        }

        Ok(())
    }

    fn download_files(&self) -> Result<()> {
        println!();
        println!("{}", "=== Downloading Files ===".cyan().bold());

        // Ensure download directory exists
        self.config.ensure_download_dir()?;

        let content_cache = ContentCache::new(self.config.contents_dir.clone());

        for file in &self.gist.files {
            // Load from cache or fetch from API
            let content = if content_cache.exists(&self.gist.id, &file.filename) {
                match content_cache.read(&self.gist.id, &file.filename) {
                    Ok(c) => c,
                    Err(_) => {
                        // Fetch from API if cache read fails
                        GitHubApi::new().fetch_gist_content(&self.gist.id, &file.filename)?
                    }
                }
            } else {
                // Fetch from API
                let fetched = GitHubApi::new().fetch_gist_content(&self.gist.id, &file.filename)?;

                // Also create cache when downloading
                let _ = content_cache.write(&self.gist.id, &file.filename, &fetched);

                fetched
            };

            // Save to download folder
            let download_path = self.config.download_dir.join(&file.filename);
            fs::write(&download_path, &content)?;

            println!(
                "{}",
                format!("  ✓ Download complete: {}", download_path.display()).green()
            );
        }

        println!();
        println!(
            "{}",
            format!(
                "Saved {} file(s) to {}",
                self.gist.files.len(),
                self.config.download_dir.display()
            )
            .green()
            .bold()
        );

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cache::types::GistFile;
    use chrono::Utc;
    use tempfile::TempDir;

    fn create_test_gist() -> GistInfo {
        GistInfo {
            id: "test123".to_string(),
            description: Some("Test gist".to_string()),
            files: vec![
                GistFile {
                    filename: "test.sh".to_string(),
                    language: Some("Shell".to_string()),
                    size: 100,
                },
                GistFile {
                    filename: "test.py".to_string(),
                    language: Some("Python".to_string()),
                    size: 200,
                },
            ],
            updated_at: Utc::now(),
            public: true,
            html_url: "https://gist.github.com/test123".to_string(),
        }
    }

    fn create_test_config() -> Config {
        let temp_dir = TempDir::new().unwrap();
        Config {
            cache_dir: temp_dir.path().to_path_buf(),
            cache_file: temp_dir.path().join("cache.json"),
            contents_dir: temp_dir.path().join("contents"),
            download_dir: temp_dir.path().join("downloads"),
        }
    }

    #[test]
    fn test_runner_new() {
        let gist = create_test_gist();
        let config = create_test_config();

        let runner = ScriptRunner::new(
            gist.clone(),
            "bash".to_string(),
            None,
            true,
            RunOptions {
                interactive: false,
                preview: false,
                download: false,
                force_file_based: false,
            },
            vec![],
            config,
        );

        assert_eq!(runner.interpreter, "bash");
        assert!(runner.is_shell);
    }

    #[test]
    fn test_run_options() {
        let options = RunOptions {
            interactive: true,
            preview: false,
            download: true,
            force_file_based: false,
        };

        assert!(options.interactive);
        assert!(!options.preview);
        assert!(options.download);
        assert!(!options.force_file_based);
    }

    #[test]
    fn test_select_main_file_single_file() {
        let config = create_test_config();
        let mut gist = create_test_gist();
        gist.files = vec![GistFile {
            filename: "single.sh".to_string(),
            language: Some("Shell".to_string()),
            size: 100,
        }];

        let runner = ScriptRunner::new(
            gist,
            "bash".to_string(),
            None,
            true,
            RunOptions {
                interactive: false,
                preview: false,
                download: false,
                force_file_based: false,
            },
            vec![],
            config,
        );

        let main_file = runner.select_main_file().unwrap();
        assert_eq!(main_file.filename, "single.sh");
    }

    #[test]
    fn test_select_main_file_multiple_files() {
        let config = create_test_config();
        let gist = create_test_gist();

        let runner = ScriptRunner::new(
            gist,
            "bash".to_string(),
            None,
            true,
            RunOptions {
                interactive: false,
                preview: false,
                download: false,
                force_file_based: false,
            },
            vec![],
            config,
        );

        let main_file = runner.select_main_file().unwrap();
        assert_eq!(main_file.filename, "test.sh"); // Matches .sh for bash
    }

    #[test]
    fn test_select_main_file_by_interpreter() {
        let config = create_test_config();
        let gist = create_test_gist();

        // Test with python interpreter
        let runner = ScriptRunner::new(
            gist,
            "python3".to_string(),
            None,
            false,
            RunOptions {
                interactive: false,
                preview: false,
                download: false,
                force_file_based: false,
            },
            vec![],
            config,
        );

        let main_file = runner.select_main_file().unwrap();
        assert_eq!(main_file.filename, "test.py"); // Matches .py for python
    }

    #[test]
    fn test_display_info() {
        let config = create_test_config();
        let gist = create_test_gist();

        let runner = ScriptRunner::new(
            gist,
            "bash".to_string(),
            None,
            true,
            RunOptions {
                interactive: false,
                preview: false,
                download: false,
                force_file_based: false,
            },
            vec![],
            config,
        );

        // Just call display_info to ensure it doesn't panic
        runner.display_info();
    }

    #[test]
    fn test_runner_with_different_interpreters() {
        let config = create_test_config();
        let gist = create_test_gist();

        // Test python3
        let runner = ScriptRunner::new(
            gist.clone(),
            "python3".to_string(),
            None,
            false,
            RunOptions {
                interactive: false,
                preview: false,
                download: false,
                force_file_based: false,
            },
            vec![],
            config.clone(),
        );
        assert_eq!(runner.interpreter, "python3");
        assert!(!runner.is_shell);

        // Test node
        let runner = ScriptRunner::new(
            gist.clone(),
            "node".to_string(),
            None,
            false,
            RunOptions {
                interactive: false,
                preview: false,
                download: false,
                force_file_based: false,
            },
            vec![],
            config.clone(),
        );
        assert_eq!(runner.interpreter, "node");

        // Test ruby
        let runner = ScriptRunner::new(
            gist.clone(),
            "ruby".to_string(),
            None,
            false,
            RunOptions {
                interactive: false,
                preview: false,
                download: false,
                force_file_based: false,
            },
            vec![],
            config.clone(),
        );
        assert_eq!(runner.interpreter, "ruby");
    }

    #[test]
    fn test_run_options_combinations() {
        let options = RunOptions {
            interactive: true,
            preview: true,
            download: true,
            force_file_based: true,
        };

        assert!(options.interactive);
        assert!(options.preview);
        assert!(options.download);
        assert!(options.force_file_based);
    }

    #[test]
    fn test_select_main_file_with_explicit_filename() {
        let config = create_test_config();
        let gist = create_test_gist();

        // Specify explicit filename - but select_main_file still uses interpreter logic
        // This test verifies the current behavior
        let runner = ScriptRunner::new(
            gist,
            "bash".to_string(),
            Some("test.py".to_string()),
            true,
            RunOptions {
                interactive: false,
                preview: false,
                download: false,
                force_file_based: false,
            },
            vec![],
            config,
        );

        // select_main_file matches by interpreter, not by main_filename
        let main_file = runner.select_main_file().unwrap();
        // For bash interpreter, it will select test.sh (matches .sh extension)
        assert_eq!(main_file.filename, "test.sh");
    }

    #[test]
    fn test_run_options_preview_mode() {
        let options = RunOptions {
            interactive: false,
            preview: true,
            download: false,
            force_file_based: false,
        };

        assert!(options.preview);
        assert!(!options.interactive);
    }

    #[test]
    fn test_run_options_download_mode() {
        let options = RunOptions {
            interactive: false,
            preview: false,
            download: true,
            force_file_based: false,
        };

        assert!(options.download);
        assert!(!options.preview);
    }
}
