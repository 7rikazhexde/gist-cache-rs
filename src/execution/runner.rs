use crate::cache::ContentCache;
use crate::cache::types::GistInfo;
use crate::config::Config;
use crate::error::{GistCacheError, Result};
use crate::github::GitHubApi;
use colored::Colorize;
use std::fs;
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::process::{Command, Stdio};

pub struct ScriptRunner {
    gist: GistInfo,
    interpreter: String,
    run_command: Option<String>,
    is_shell: bool,
    interactive: bool,
    preview: bool,
    force_file_based: bool,
    args: Vec<String>,
    config: Config,
}

impl ScriptRunner {
    pub fn new(
        gist: GistInfo,
        interpreter: String,
        run_command: Option<String>,
        is_shell: bool,
        interactive: bool,
        preview: bool,
        force_file_based: bool,
        args: Vec<String>,
        config: Config,
    ) -> Self {
        Self {
            gist,
            interpreter,
            run_command,
            is_shell,
            interactive,
            preview,
            force_file_based,
            args,
            config,
        }
    }

    pub fn run(&self) -> Result<()> {
        // Display gist info
        self.display_info();

        if self.preview {
            return self.preview_content();
        }

        self.execute()
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
        println!("{}", "=== Gist内容 ===".cyan().bold());

        for file in &self.gist.files {
            println!("\n{}", format!("--- {} ---", file.filename).yellow().bold());

            // キャッシュチェック
            let content_cache = ContentCache::new(self.config.contents_dir.clone());

            let content = if content_cache.exists(&self.gist.id, &file.filename) {
                // キャッシュから読み込み
                match content_cache.read(&self.gist.id, &file.filename) {
                    Ok(c) => c,
                    Err(_) => {
                        // キャッシュ読み込み失敗時はAPIから取得
                        GitHubApi::fetch_gist_content(&self.gist.id, &file.filename)?
                    }
                }
            } else {
                // APIから取得
                GitHubApi::fetch_gist_content(&self.gist.id, &file.filename)?
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
            format!("実行中: {} ({})", main_file.filename, self.interpreter).cyan()
        );

        // キャッシュチェックと本文取得
        let content_cache = ContentCache::new(self.config.contents_dir.clone());

        let content = if content_cache.exists(&self.gist.id, &main_file.filename) {
            // キャッシュから読み込み
            match content_cache.read(&self.gist.id, &main_file.filename) {
                Ok(c) => {
                    if std::env::var("GIST_CACHE_VERBOSE").is_ok() {
                        println!("{}", "  → キャッシュからロード".green());
                    }
                    c
                }
                Err(e) => {
                    // 自己修復の原則：キャッシュ読み込み失敗時はAPIから取得
                    eprintln!(
                        "{}",
                        format!("  警告: キャッシュ読み込み失敗、APIから取得します: {}", e)
                            .yellow()
                    );
                    let fetched =
                        GitHubApi::fetch_gist_content(&self.gist.id, &main_file.filename)?;

                    // 取得に成功したらキャッシュに保存を試みる
                    let _ = content_cache.write(&self.gist.id, &main_file.filename, &fetched);

                    fetched
                }
            }
        } else {
            // APIから取得
            println!(
                "{}",
                "  情報: キャッシュが存在しないため、GitHub APIから取得します...".yellow()
            );
            let fetched = GitHubApi::fetch_gist_content(&self.gist.id, &main_file.filename)?;
            fetched
        };

        // 対話モードでの整合性確保：
        // キャッシュから読み込む場合もAPIから取得する場合も、
        // 常に一時ファイルを経由して実行することで動作を統一
        let execution_result = if self.force_file_based || self.interactive || self.is_shell {
            self.execute_via_temp_file(&content, &main_file.filename)
        } else {
            self.execute_direct(&content)
        };

        // 実行が成功した場合のみキャッシュに保存
        if execution_result.is_ok() {
            // キャッシュが存在しない場合のみ保存（既存のキャッシュは上書きしない）
            if !content_cache.exists(&self.gist.id, &main_file.filename) {
                match content_cache.write(&self.gist.id, &main_file.filename, &content) {
                    Ok(_) => {
                        if std::env::var("GIST_CACHE_VERBOSE").is_ok() {
                            println!(
                                "{}",
                                format!(
                                    "  → キャッシュに保存しました: {}",
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
                        // キャッシュ保存失敗は警告のみ（実行は成功しているため）
                        eprintln!(
                            "{}",
                            format!("  警告: キャッシュ保存に失敗: {}", e).yellow()
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

        if !preferred_extension.is_empty() {
            if let Some(file) = self
                .gist
                .files
                .iter()
                .find(|f| f.filename.ends_with(preferred_extension))
            {
                return Ok(file);
            }
        }

        // Default to first file
        Ok(&self.gist.files[0])
    }

    /// 一時ファイル経由での実行（対話モード、シェルスクリプト、file-basedインタープリタ）
    ///
    /// 重要：キャッシュの有無に関わらず、この関数を使用することで
    /// 対話的なスクリプトの動作が一貫することを保証
    fn execute_via_temp_file(&self, content: &str, filename: &str) -> Result<()> {
        // Create temporary file
        let temp_dir = std::env::temp_dir();
        let temp_file = temp_dir.join(filename);

        fs::write(&temp_file, content)?;

        // Make executable for shell scripts
        if self.is_shell {
            let mut perms = fs::metadata(&temp_file)?.permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&temp_file, perms)?;
        }

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
            command.arg(&temp_file);
            command
        };

        // Add user arguments
        for arg in &self.args {
            cmd.arg(arg);
        }

        // Run with inherited stdio for interactive mode
        // 対話モードでは標準入力を継承することで、readコマンドなどが正常に動作
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

    /// 標準入力経由での直接実行（非対話モード、stdin対応インタープリタ）
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
}
