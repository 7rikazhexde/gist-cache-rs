use crate::cache::types::GistInfo;
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
            let content = GitHubApi::fetch_gist_content(&self.gist.id, &file.filename)?;
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

        // Fetch content
        let content = GitHubApi::fetch_gist_content(&self.gist.id, &main_file.filename)?;

        // Execute based on mode - force file-based for uv/poetry/PHP
        if self.force_file_based || self.interactive || self.is_shell {
            self.execute_interactive(&content, &main_file.filename)
        } else {
            self.execute_direct(&content)
        }
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

    fn execute_interactive(&self, content: &str, filename: &str) -> Result<()> {
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
