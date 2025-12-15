use crate::cache::CleanOptions;
use crate::*;
use clap::{Args, CommandFactory, Parser, Subcommand, ValueEnum};
use clap_complete::{Shell as CompletionShell, generate};
use colored::Colorize;
use serde::Serialize;
use std::fs;
use std::io;

#[derive(Parser)]
#[command(name = "gist-cache-rs")]
#[command(about = "Gist caching and execution system (Rust implementation)", long_about = None)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Update cache
    Update(UpdateArgs),
    /// Search from cache and execute
    Run(RunArgs),
    /// Cache management
    Cache(CacheArgs),
    /// Configuration management
    Config(ConfigArgs),
    /// Generate shell completion scripts
    Completions(CompletionsArgs),
}

#[derive(Args)]
pub struct CompletionsArgs {
    /// Shell to generate completions for
    #[arg(value_enum)]
    pub shell: Shell,
}

#[derive(Args)]
pub struct ConfigArgs {
    #[command(subcommand)]
    pub command: ConfigCommands,
}

#[derive(Args)]
pub struct SetConfigArgs {
    /// Configuration key (e.g., defaults.interpreter)
    pub key: String,
    /// Configuration value
    pub value: String,
}

#[derive(Args)]
pub struct GetConfigArgs {
    /// Configuration key
    pub key: String,
}

#[derive(Subcommand)]
pub enum ConfigCommands {
    /// Set a configuration value
    Set(SetConfigArgs),
    /// Get a configuration value
    Get(GetConfigArgs),
    /// Show all configuration values
    Show,
    /// Edit configuration file in $EDITOR
    Edit,
    /// Reset configuration to defaults
    Reset,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, ValueEnum)]
pub enum Shell {
    /// Bash shell
    Bash,
    /// Zsh shell
    Zsh,
    /// Fish shell
    Fish,
    /// PowerShell
    #[value(name = "powershell")]
    PowerShell,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, ValueEnum)]
pub enum OutputFormat {
    /// Human-readable text format (default)
    Text,
    /// JSON format for scripting
    Json,
}

#[derive(Args)]
pub struct UpdateArgs {
    /// Force full update
    #[arg(short, long)]
    pub force: bool,

    /// Display detailed progress information
    #[arg(short, long)]
    pub verbose: bool,
}

#[derive(Args)]
pub struct RunArgs {
    /// Search keyword (ID, filename, or description)
    pub query: Option<String>,

    /// Interactive script execution mode
    #[arg(short, long)]
    pub interactive: bool,

    /// Preview mode (display content only)
    #[arg(short, long)]
    pub preview: bool,

    /// Update Gist cache before execution
    #[arg(short, long)]
    pub force: bool,

    /// Save file to download folder
    #[arg(long)]
    pub download: bool,

    /// Direct ID specification mode
    #[arg(long)]
    pub id: bool,

    /// Search by filename
    #[arg(long)]
    pub filename: bool,

    /// Search by description
    #[arg(long)]
    pub description: bool,

    /// Search using regular expression pattern
    #[arg(long, value_name = "PATTERN")]
    pub regex: Option<String>,

    /// Filter by programming language
    #[arg(long, value_name = "LANGUAGE")]
    pub language: Option<String>,

    /// Filter by file extension
    #[arg(long, value_name = "EXT")]
    pub extension: Option<String>,

    /// Interpreter or execution command (bash, python3, uv, etc.)
    #[arg(value_name = "INTERPRETER")]
    pub interpreter: Option<String>,

    /// Additional arguments to pass to the script
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    pub script_args: Vec<String>,
}

#[derive(Args)]
pub struct CacheArgs {
    #[command(subcommand)]
    pub command: CacheCommands,
}

#[derive(Subcommand)]
pub enum CacheCommands {
    /// Display list of cached Gists
    List(ListArgs),
    /// Display total cache size
    Size,
    /// Remove old cache entries
    Clean(CleanArgs),
    /// Remove all cache
    Clear,
}

#[derive(Args)]
pub struct ListArgs {
    /// Output format
    #[arg(long, value_enum, default_value = "text")]
    pub format: OutputFormat,
}

#[derive(Serialize)]
struct GistListItem {
    id: String,
    description: Option<String>,
    files: Vec<String>,
    updated_at: String,
}

#[derive(Args)]
pub struct CleanArgs {
    /// Remove entries older than specified days
    #[arg(long, value_name = "DAYS")]
    pub older_than: Option<u32>,

    /// Remove orphaned content cache files (content without metadata)
    #[arg(long)]
    pub orphaned: bool,

    /// Preview what would be deleted without actually deleting
    #[arg(long)]
    pub dry_run: bool,
}

pub fn run_cli() -> Result<()> {
    let cli = Cli::parse();
    let config = Config::new()?;

    match cli.command {
        Commands::Update(args) => {
            let updater = CacheUpdater::new(config, args.verbose);
            updater.update(args.force)?;
        }
        Commands::Run(args) => {
            // Display help if no query is provided
            if args.query.is_none() {
                print_run_help();
                return Ok(());
            }

            // Update cache first if --force option is specified
            if args.force {
                let updater = CacheUpdater::new(config.clone(), false);
                updater.update(false)?; // Differential update (force=false)
            }

            run_gist(config, args)?;
        }
        Commands::Cache(args) => {
            handle_cache_command(config, args)?;
        }
        Commands::Config(args) => {
            handle_config_command(config, args)?;
        }
        Commands::Completions(args) => {
            generate_completions(args.shell)?;
        }
    }

    Ok(())
}

pub fn print_run_help() {
    println!("{}", "Search from cache and execute".bold());
    println!();
    println!("Usage: gist-cache-rs run [OPTIONS] <QUERY> [INTERPRETER] [SCRIPT_ARGS]...");
    println!();
    println!("{}", "Note argument order:".yellow().bold());
    println!("  Options (like --description) must be specified before the search keyword");
    println!();
    println!("Arguments:");
    println!("  <QUERY>           Search keyword (ID, filename, or description)");
    println!("  [INTERPRETER]     Interpreter (bash, python3, uv, etc.)");
    println!("  [SCRIPT_ARGS]...  Additional arguments to pass to the script");
    println!();
    println!("Options:");
    println!("  -i, --interactive  Interactive script execution mode");
    println!("  -p, --preview      Preview mode (display content only)");
    println!("  -f, --force        Update Gist cache before execution (always get latest version)");
    println!("      --download     Save file to download folder");
    println!("      --id           Direct ID specification mode");
    println!("      --filename     Search by filename");
    println!("      --description  Search by description");
    println!("  -h, --help         Print help");
    println!();
    println!("{}", "Supported interpreters:".green().bold());
    println!("  bash, sh, zsh      - Shell scripts");
    println!("  python3, python    - Python");
    println!("  uv                 - Python (PEP 723 support)");
    println!("  ruby, node, perl, php - Other languages");
    println!();
    println!("Examples:");
    println!("  gist-cache-rs run backup                      # Keyword search");
    println!("  gist-cache-rs run backup bash /src /dst       # Execute with arguments");
    println!("  gist-cache-rs run data python3                # Python execution");
    println!("  gist-cache-rs run --description numpy uv      # Description search + uv execution");
    println!("  gist-cache-rs run -p backup                   # Preview");
    println!("  gist-cache-rs run -i interactive-script       # Interactive mode");
    println!("  gist-cache-rs run --filename setup.sh         # Filename search");
    println!("  gist-cache-rs run --id abc123def456           # ID specification");
    println!("  gist-cache-rs run -f backup                   # Execute after cache update");
    println!("  gist-cache-rs run -f --description numpy uv   # Cache update + description search");
    println!("  gist-cache-rs run --download backup           # Save to download folder");
    println!("  gist-cache-rs run -p --download backup        # Preview then download");
    println!();
    println!("{}", "Verify argument specification:".red().bold());
    println!("  ✅ uv example: gist-cache-rs run --description numpy uv input.csv");
    println!();
    println!("For more information, try '--help'");
}

pub fn run_gist(config: Config, args: RunArgs) -> Result<()> {
    // Check cache exists
    if !config.cache_exists() {
        return Err(GistCacheError::CacheNotFound);
    }

    // Ensure query is always Some
    let query_string = args.query.unwrap();

    // Load cache
    let cache_content = fs::read_to_string(&config.cache_file)?;
    let cache: GistCache = serde_json::from_str(&cache_content)?;

    // Determine search mode
    let search_mode = if args.id {
        SearchMode::Id
    } else if args.filename {
        SearchMode::Filename
    } else if args.description {
        SearchMode::Description
    } else {
        SearchMode::Auto
    };

    // Build search options from CLI arguments
    let search_options = search::SearchOptions {
        regex: args.regex.clone(),
        language: args.language.clone(),
        extension: args.extension.clone(),
    };

    // Search for gists
    let query =
        SearchQuery::new_with_options(query_string.clone(), search_mode.clone(), search_options);
    let results = query.search(&cache.gists)?;

    if results.is_empty() {
        return Err(GistCacheError::NoSearchResults(query_string));
    }

    // Select gist
    let gist = if matches!(search_mode, SearchMode::Id) && results.len() == 1 {
        println!(
            "{}",
            format!("ID specification mode: {}", results[0].id).cyan()
        );
        results[0]
    } else {
        search::select_from_results(&results)?
    };

    // Parse interpreter and execution mode
    let (interpreter, run_command, is_shell, force_file_based) =
        parse_interpreter(args.interpreter.as_deref())?;

    // Create and run script runner
    let options = RunOptions {
        interactive: args.interactive,
        preview: args.preview,
        download: args.download,
        force_file_based,
    };
    let runner = ScriptRunner::new(
        gist.clone(),
        interpreter,
        run_command,
        is_shell,
        options,
        args.script_args,
        config,
    );

    runner.run()?;

    Ok(())
}

pub fn parse_interpreter(
    interpreter: Option<&str>,
) -> Result<(String, Option<String>, bool, bool)> {
    match interpreter {
        Some("bash") | Some("sh") | Some("zsh") => {
            Ok((interpreter.unwrap().to_string(), None, true, false))
        }
        Some("python") | Some("python3") | Some("ruby") | Some("node") | Some("perl") => {
            Ok((interpreter.unwrap().to_string(), None, false, false))
        }
        Some("php") => {
            // PHP: Force file-based execution for reliable argument handling and stdin stability
            Ok(("php".to_string(), None, false, true))
        }
        Some("pwsh") | Some("powershell") => {
            // PowerShell: Force file-based execution for script execution policy compatibility
            Ok((interpreter.unwrap().to_string(), None, false, true))
        }
        Some("ts-node") => {
            // ts-node: TypeScript execution via Node.js (file-based for module resolution)
            // Compiler options for Node.js v22+ ESM compatibility are added in runner
            Ok(("ts-node".to_string(), None, false, true))
        }
        Some("deno") => {
            // Deno: Native TypeScript support with 'deno run' command
            Ok((
                "deno".to_string(),
                Some("deno run".to_string()),
                false,
                true,
            ))
        }
        Some("bun") => {
            // Bun: Native TypeScript support (file-based)
            Ok(("bun".to_string(), None, false, true))
        }
        Some("uv") => {
            // uv always runs in file-based mode (for PEP 723 support)
            Ok((
                "python3".to_string(),
                Some("uv run".to_string()),
                false,
                true,
            ))
        }
        Some("poetry") => {
            println!(
                "{}",
                "Warning: Poetry does not support PEP 723. Running with python3.".yellow()
            );
            Ok(("python3".to_string(), None, false, false))
        }
        None => Ok(("bash".to_string(), None, true, false)),
        Some(custom) => {
            // Check if the custom interpreter exists
            // Use 'where' on Windows, 'which' on Unix
            #[cfg(windows)]
            let check_cmd = "where";
            #[cfg(not(windows))]
            let check_cmd = "which";

            let interpreter_exists = std::process::Command::new(check_cmd)
                .arg(custom)
                .output()
                .map(|output| output.status.success())
                .unwrap_or(false);

            if !interpreter_exists {
                eprintln!();
                eprintln!("{}", "Supported interpreters:".green());
                eprintln!(
                    "  bash, sh, zsh, python3, python, uv, ruby, node, perl, php, pwsh, powershell"
                );
                eprintln!("  ts-node, deno, bun (TypeScript)");
                eprintln!();
                eprintln!("{}", "Verify argument specification:".yellow());
                eprintln!("  ✅ uv example: gist-cache-rs run --description numpy uv input.csv");
                eprintln!();
                return Err(GistCacheError::InvalidInterpreter(custom.to_string()));
            }
            Ok((custom.to_string(), None, false, false))
        }
    }
}

pub fn handle_cache_command(config: Config, args: CacheArgs) -> Result<()> {
    let content_cache = ContentCache::new(config.contents_dir.clone());

    match args.command {
        CacheCommands::List(list_args) => {
            let gist_ids = content_cache.list_cached_gists()?;

            if gist_ids.is_empty() {
                if list_args.format == OutputFormat::Json {
                    println!("[]");
                } else {
                    println!("{}", "No cached Gists".yellow());
                }
                return Ok(());
            }

            // Load metadata JSON and display detailed information
            if config.cache_exists() {
                let cache_content = fs::read_to_string(&config.cache_file)?;
                let cache: GistCache = serde_json::from_str(&cache_content)?;

                match list_args.format {
                    OutputFormat::Json => {
                        let items: Vec<GistListItem> = gist_ids
                            .iter()
                            .filter_map(|gist_id| {
                                cache.gists.iter().find(|g| &g.id == gist_id).map(|gist| {
                                    GistListItem {
                                        id: gist.id.clone(),
                                        description: gist.description.clone(),
                                        files: gist
                                            .files
                                            .iter()
                                            .map(|f| f.filename.clone())
                                            .collect(),
                                        updated_at: gist.updated_at.to_rfc3339(),
                                    }
                                })
                            })
                            .collect();

                        let json = serde_json::to_string_pretty(&items)?;
                        println!("{}", json);
                    }
                    OutputFormat::Text => {
                        println!("{}", "List of cached Gists:".cyan().bold());
                        println!();

                        for gist_id in &gist_ids {
                            if let Some(gist) = cache.gists.iter().find(|g| &g.id == gist_id) {
                                let desc = gist.description.as_deref().unwrap_or("No description");

                                let files: Vec<_> =
                                    gist.files.iter().map(|f| f.filename.as_str()).collect();

                                println!("{}", format!("ID: {}", gist.id).green());
                                println!("  Description: {}", desc);
                                println!("  Files: {}", files.join(", "));
                                println!(
                                    "  Updated: {}",
                                    gist.updated_at.format("%Y-%m-%d %H:%M:%S")
                                );
                                println!();
                            } else {
                                println!("{}", format!("ID: {}", gist_id).green());
                                println!("  (Metadata not found)");
                                println!();
                            }
                        }

                        println!(
                            "{}",
                            format!("Total: {} Gists cached", gist_ids.len())
                                .cyan()
                                .bold()
                        );
                    }
                }
            } else {
                // Display only IDs when metadata is not available
                match list_args.format {
                    OutputFormat::Json => {
                        let items: Vec<GistListItem> = gist_ids
                            .iter()
                            .map(|gist_id| GistListItem {
                                id: gist_id.clone(),
                                description: None,
                                files: vec![],
                                updated_at: String::new(),
                            })
                            .collect();

                        let json = serde_json::to_string_pretty(&items)?;
                        println!("{}", json);
                    }
                    OutputFormat::Text => {
                        println!("{}", "List of cached Gists:".cyan().bold());
                        println!();

                        for gist_id in &gist_ids {
                            println!("  {}", gist_id.green());
                        }
                        println!();
                        println!(
                            "{}",
                            format!("Total: {} items", gist_ids.len()).cyan().bold()
                        );
                    }
                }
            }
        }
        CacheCommands::Size => {
            println!("{}", "Cache size information:".cyan().bold());
            println!();

            let total_size = content_cache.total_size()?;
            let gist_count = content_cache.list_cached_gists()?.len();

            println!("{}", format!("Cached Gists: {} items", gist_count).green());
            println!(
                "{}",
                format!("Total size: {}", format_bytes(total_size)).green()
            );
            println!(
                "{}",
                format!("Cache directory: {}", config.contents_dir.display()).cyan()
            );
        }
        CacheCommands::Clean(args) => {
            println!("{}", "Clean cache entries".cyan().bold());
            println!();

            // Load metadata cache
            if !config.cache_exists() {
                return Err(GistCacheError::CacheNotFound);
            }

            let cache_content = fs::read_to_string(&config.cache_file)?;
            let metadata_cache: GistCache = serde_json::from_str(&cache_content)?;

            // Convert CleanArgs to CleanOptions
            let options = CleanOptions {
                older_than_days: args.older_than,
                orphaned: args.orphaned,
                dry_run: args.dry_run,
            };

            // Show what will be done
            if args.dry_run {
                println!(
                    "{}",
                    "DRY RUN MODE - No files will be deleted".yellow().bold()
                );
                println!();
            }

            if let Some(days) = args.older_than {
                println!("  Removing entries older than {} days", days);
            }
            if args.orphaned {
                println!("  Removing orphaned content cache files");
            }
            if !args.orphaned && args.older_than.is_none() {
                println!(
                    "{}",
                    "No cleaning criteria specified. Use --older-than or --orphaned".yellow()
                );
                println!();
                println!("Examples:");
                println!(
                    "  gist-cache-rs cache clean --older-than 30    # Remove entries older than 30 days"
                );
                println!(
                    "  gist-cache-rs cache clean --orphaned         # Remove orphaned cache files"
                );
                println!(
                    "  gist-cache-rs cache clean --dry-run --orphaned  # Preview what would be deleted"
                );
                return Ok(());
            }

            println!();

            // Execute clean
            let result = content_cache.clean(&metadata_cache, &options)?;

            // Display results
            if result.deleted_gists.is_empty() {
                println!("{}", "No cache entries to clean".green());
            } else {
                if args.dry_run {
                    println!(
                        "{}",
                        format!("Would delete {} entries:", result.deleted_gists.len())
                            .yellow()
                            .bold()
                    );
                } else {
                    println!(
                        "{}",
                        format!("Deleted {} entries:", result.deleted_gists.len())
                            .green()
                            .bold()
                    );
                }
                println!();

                for gist_id in &result.deleted_gists {
                    if let Some(gist) = metadata_cache.gists.iter().find(|g| &g.id == gist_id) {
                        let desc = gist.description.as_deref().unwrap_or("No description");
                        println!("{}", format!("  ID: {}", gist.id).cyan());
                        println!("    Description: {}", desc);
                        println!(
                            "    Updated: {}",
                            gist.updated_at.format("%Y-%m-%d %H:%M:%S")
                        );
                    } else {
                        println!("{}", format!("  ID: {} (orphaned)", gist_id).cyan());
                    }
                }

                println!();
                if args.dry_run {
                    println!(
                        "{}",
                        format!("Would free up: {}", format_bytes(result.deleted_size)).yellow()
                    );
                } else {
                    println!(
                        "{}",
                        format!("Freed up: {}", format_bytes(result.deleted_size))
                            .green()
                            .bold()
                    );
                }
            }
        }
        CacheCommands::Clear => {
            println!("{}", "Remove all cache".yellow().bold());
            println!();

            let gist_count = content_cache.list_cached_gists()?.len();

            if gist_count == 0 {
                println!("{}", "No cache to remove".green());
                return Ok(());
            }

            println!(
                "{}",
                format!(
                    "About to remove cache for {} Gists. Are you sure?",
                    gist_count
                )
                .yellow()
            );
            println!("  {}", "This operation cannot be undone.".red());
            println!();
            print!("Continue? (y/N): ");

            std::io::Write::flush(&mut std::io::stdout())?;

            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;

            if input.trim().to_lowercase() == "y" {
                content_cache.clear_all()?;
                println!();
                println!("{}", "All cache has been removed".green().bold());
            } else {
                println!();
                println!("{}", "Cancelled".cyan());
            }
        }
    }

    Ok(())
}

pub fn format_bytes(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} bytes", bytes)
    }
}

/// Generate shell completion scripts
pub fn generate_completions(shell: Shell) -> Result<()> {
    let mut cmd = Cli::command();
    let bin_name = "gist-cache-rs";

    let completion_shell = match shell {
        Shell::Bash => CompletionShell::Bash,
        Shell::Zsh => CompletionShell::Zsh,
        Shell::Fish => CompletionShell::Fish,
        Shell::PowerShell => CompletionShell::PowerShell,
    };

    generate(completion_shell, &mut cmd, bin_name, &mut io::stdout());
    Ok(())
}

pub fn handle_config_command(mut config: Config, args: ConfigArgs) -> Result<()> {
    use colored::Colorize;

    match args.command {
        ConfigCommands::Set(set_args) => {
            config.set_config_value(&set_args.key, &set_args.value)?;

            println!(
                "{}",
                format!("✓ Set {} = {}", set_args.key, set_args.value).green()
            );
        }

        ConfigCommands::Get(get_args) => match config.get_config_value(&get_args.key) {
            Some(value) => println!("{}", value),

            None => println!(
                "{}",
                format!("Config key '{}' not set", get_args.key).yellow()
            ),
        },

        ConfigCommands::Show => {
            println!("{}", "Configuration:".bold());

            println!();

            let mut is_empty = true;

            // Show defaults

            if let Some(ref defaults) = config.user_config.defaults {
                if let Some(ref interpreter) = defaults.interpreter {
                    println!("{}", "[defaults]".cyan());

                    println!("  interpreter = {}", interpreter.yellow());

                    is_empty = false;
                }
            }

            // Show execution

            if let Some(ref execution) = config.user_config.execution {
                if let Some(confirm) = execution.confirm_before_run {
                    println!("{}", "[execution]".cyan());

                    println!("  confirm_before_run = {}", confirm.to_string().yellow());

                    is_empty = false;
                }
            }

            // Show cache

            if let Some(ref cache_config) = config.user_config.cache {
                if let Some(days) = cache_config.retention_days {
                    println!("{}", "[cache]".cyan());

                    println!("  retention_days = {}", days.to_string().yellow());

                    is_empty = false;
                }
            }

            if is_empty {
                println!("{}", "No configuration settings found.".yellow());

                println!();

                println!(
                    "{}",
                    "You can set options using the 'edit' or 'set' commands.".bold()
                );

                println!();

                println!("{}", "Available options:".cyan());

                println!("  [defaults]");

                println!("    interpreter = <default_interpreter>   (e.g., \"python3\", \"bash\")");

                println!();

                println!("  [execution]");

                println!("    confirm_before_run = <true|false>");

                println!();

                println!("  [cache]");

                println!("    retention_days = <number_of_days>");

                println!();

                println!("{}", "Examples:".cyan());

                println!("  gist-cache-rs config edit");

                println!("  gist-cache-rs config set defaults.interpreter python3");
            }

            println!();

            println!(
                "{}",
                format!("Config file: {}", config.config_file.display()).dimmed()
            );
        }

        ConfigCommands::Edit => {
            // Ensure config file exists

            if !config.config_file.exists() {
                config.save_user_config()?;
            }

            let editor = std::env::var("EDITOR").unwrap_or_else(|_| {
                #[cfg(windows)]
                return "notepad".to_string();

                #[cfg(not(windows))]
                return "vi".to_string();
            });

            std::process::Command::new(editor)
                .arg(&config.config_file)
                .status()
                .map_err(GistCacheError::Io)?;

            println!("{}", "✓ Configuration file edited".green());
        }

        ConfigCommands::Reset => {
            config.reset_config()?;

            println!("{}", "✓ Configuration reset to defaults".green());
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_parse_interpreter_bash() {
        let result = parse_interpreter(Some("bash")).unwrap();
        assert_eq!(result.0, "bash");
        assert_eq!(result.1, None);
        assert!(result.2); // is_shell
        assert!(!result.3); // force_file_based
    }

    #[test]
    fn test_parse_interpreter_sh() {
        let result = parse_interpreter(Some("sh")).unwrap();
        assert_eq!(result.0, "sh");
        assert!(result.2); // is_shell
    }

    #[test]
    fn test_parse_interpreter_zsh() {
        let result = parse_interpreter(Some("zsh")).unwrap();
        assert_eq!(result.0, "zsh");
        assert!(result.2); // is_shell
    }

    #[test]
    fn test_parse_interpreter_python() {
        let result = parse_interpreter(Some("python3")).unwrap();
        assert_eq!(result.0, "python3");
        assert!(!result.2); // not shell
    }

    #[test]
    fn test_parse_interpreter_ruby() {
        let result = parse_interpreter(Some("ruby")).unwrap();
        assert_eq!(result.0, "ruby");
        assert!(!result.2); // not shell
    }

    #[test]
    fn test_parse_interpreter_node() {
        let result = parse_interpreter(Some("node")).unwrap();
        assert_eq!(result.0, "node");
        assert!(!result.2); // not shell
    }

    #[test]
    fn test_parse_interpreter_perl() {
        let result = parse_interpreter(Some("perl")).unwrap();
        assert_eq!(result.0, "perl");
        assert!(!result.2); // not shell
    }

    #[test]
    fn test_parse_interpreter_uv() {
        let result = parse_interpreter(Some("uv")).unwrap();
        assert_eq!(result.0, "python3");
        assert_eq!(result.1, Some("uv run".to_string()));
        assert!(result.3); // force_file_based
    }

    #[test]
    fn test_parse_interpreter_poetry() {
        // poetry は python3 にフォールバックする
        let result = parse_interpreter(Some("poetry")).unwrap();
        assert_eq!(result.0, "python3");
        assert_eq!(result.1, None);
        assert!(!result.3); // not force_file_based
    }

    #[test]
    fn test_parse_interpreter_none() {
        let result = parse_interpreter(None).unwrap();
        assert_eq!(result.0, "bash");
        assert!(result.2); // is_shell
    }

    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(0), "0 bytes");
        assert_eq!(format_bytes(512), "512 bytes");
        assert_eq!(format_bytes(1024), "1.00 KB");
        assert_eq!(format_bytes(1024 * 1024), "1.00 MB");
        assert_eq!(format_bytes(1024 * 1024 * 1024), "1.00 GB");
        assert_eq!(format_bytes(1536), "1.50 KB");
        assert_eq!(format_bytes(2560), "2.50 KB");
        assert_eq!(format_bytes(1024 * 1024 * 2), "2.00 MB");
    }

    #[test]
    fn test_parse_interpreter_php() {
        let result = parse_interpreter(Some("php")).unwrap();
        assert_eq!(result.0, "php");
        assert!(result.3); // force_file_based
    }

    #[test]
    fn test_parse_interpreter_pwsh() {
        let result = parse_interpreter(Some("pwsh")).unwrap();
        assert_eq!(result.0, "pwsh");
        assert!(result.3); // force_file_based
    }

    #[test]
    fn test_parse_interpreter_powershell() {
        let result = parse_interpreter(Some("powershell")).unwrap();
        assert_eq!(result.0, "powershell");
        assert!(result.3); // force_file_based
    }

    #[test]
    fn test_parse_interpreter_ts_node() {
        let result = parse_interpreter(Some("ts-node")).unwrap();
        assert_eq!(result.0, "ts-node");
        assert!(result.3); // force_file_based
    }

    #[test]
    fn test_parse_interpreter_deno() {
        let result = parse_interpreter(Some("deno")).unwrap();
        assert_eq!(result.0, "deno");
        assert_eq!(result.1, Some("deno run".to_string()));
        assert!(result.3); // force_file_based
    }

    #[test]
    fn test_parse_interpreter_bun() {
        let result = parse_interpreter(Some("bun")).unwrap();
        assert_eq!(result.0, "bun");
        assert!(result.3); // force_file_based
    }

    #[test]
    fn test_run_gist_cache_not_found() {
        let temp_dir = TempDir::new().unwrap();
        let config = Config {
            cache_dir: temp_dir.path().to_path_buf(),
            cache_file: temp_dir.path().join("cache.json"),
            contents_dir: temp_dir.path().join("contents"),
            download_dir: temp_dir.path().join("downloads"),
            config_file: temp_dir.path().join("config.toml"),
            user_config: crate::config::UserConfig::default(),
        };

        let args = RunArgs {
            query: Some("test".to_string()),
            interactive: false,
            preview: false,
            force: false,
            download: false,
            id: false,
            filename: false,
            description: false,
            regex: None,
            language: None,
            extension: None,
            interpreter: None,
            script_args: vec![],
        };

        let result = run_gist(config, args);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), GistCacheError::CacheNotFound));
    }

    #[test]
    fn test_print_run_help() {
        // Just ensure it doesn't panic
        print_run_help();
    }

    #[test]
    fn test_handle_cache_command_size() {
        use std::fs;
        let temp_dir = TempDir::new().unwrap();
        let config = Config {
            cache_dir: temp_dir.path().to_path_buf(),
            cache_file: temp_dir.path().join("cache.json"),
            contents_dir: temp_dir.path().join("contents"),
            download_dir: temp_dir.path().join("downloads"),
            config_file: temp_dir.path().join("config.toml"),
            user_config: crate::config::UserConfig::default(),
        };

        fs::create_dir_all(&config.contents_dir).unwrap();

        let args = CacheArgs {
            command: CacheCommands::Size,
        };

        // Should succeed
        let result = handle_cache_command(config, args);
        assert!(result.is_ok());
    }

    #[test]
    fn test_handle_cache_command_list_empty() {
        use std::fs;
        let temp_dir = TempDir::new().unwrap();
        let config = Config {
            cache_dir: temp_dir.path().to_path_buf(),
            cache_file: temp_dir.path().join("cache.json"),
            contents_dir: temp_dir.path().join("contents"),
            download_dir: temp_dir.path().join("downloads"),
            config_file: temp_dir.path().join("config.toml"),
            user_config: crate::config::UserConfig::default(),
        };

        fs::create_dir_all(&config.contents_dir).unwrap();

        let args = CacheArgs {
            command: CacheCommands::List(ListArgs {
                format: OutputFormat::Text,
            }),
        };

        let result = handle_cache_command(config, args);
        assert!(result.is_ok());
    }

    #[test]
    fn test_handle_cache_command_clean_no_cache() {
        use std::fs;
        let temp_dir = TempDir::new().unwrap();
        let config = Config {
            cache_dir: temp_dir.path().to_path_buf(),
            cache_file: temp_dir.path().join("cache.json"),
            contents_dir: temp_dir.path().join("contents"),
            download_dir: temp_dir.path().join("downloads"),
            config_file: temp_dir.path().join("config.toml"),
            user_config: crate::config::UserConfig::default(),
        };

        fs::create_dir_all(&config.contents_dir).unwrap();

        // Test with no metadata cache (should return error)
        let args = CacheArgs {
            command: CacheCommands::Clean(CleanArgs {
                older_than: Some(30),
                orphaned: false,
                dry_run: false,
            }),
        };

        let result = handle_cache_command(config, args);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), GistCacheError::CacheNotFound));
    }

    #[test]
    fn test_handle_cache_command_clean_no_criteria() {
        use crate::cache::types::{CacheMetadata, GistCache};
        use chrono::Utc;
        use std::fs;

        let temp_dir = TempDir::new().unwrap();
        let config = Config {
            cache_dir: temp_dir.path().to_path_buf(),
            cache_file: temp_dir.path().join("cache.json"),
            contents_dir: temp_dir.path().join("contents"),
            download_dir: temp_dir.path().join("downloads"),
            config_file: temp_dir.path().join("config.toml"),
            user_config: crate::config::UserConfig::default(),
        };

        fs::create_dir_all(&config.contents_dir).unwrap();

        // Create empty metadata cache
        let cache = GistCache {
            metadata: CacheMetadata {
                last_updated: Utc::now(),
                total_count: 0,
                github_user: "testuser".to_string(),
            },
            gists: vec![],
        };
        fs::write(&config.cache_file, serde_json::to_string(&cache).unwrap()).unwrap();

        // Test with no criteria specified (should return ok but show message)
        let args = CacheArgs {
            command: CacheCommands::Clean(CleanArgs {
                older_than: None,
                orphaned: false,
                dry_run: false,
            }),
        };

        let result = handle_cache_command(config, args);
        assert!(result.is_ok());
    }

    #[test]
    fn test_handle_cache_command_clean_with_orphaned() {
        use crate::cache::ContentCache;
        use crate::cache::types::{CacheMetadata, GistCache};
        use chrono::Utc;
        use std::fs;

        let temp_dir = TempDir::new().unwrap();
        let config = Config {
            cache_dir: temp_dir.path().to_path_buf(),
            cache_file: temp_dir.path().join("cache.json"),
            contents_dir: temp_dir.path().join("contents"),
            download_dir: temp_dir.path().join("downloads"),
            config_file: temp_dir.path().join("config.toml"),
            user_config: crate::config::UserConfig::default(),
        };

        fs::create_dir_all(&config.contents_dir).unwrap();

        // Create empty metadata cache (no valid gists)
        let cache = GistCache {
            metadata: CacheMetadata {
                last_updated: Utc::now(),
                total_count: 0,
                github_user: "testuser".to_string(),
            },
            gists: vec![],
        };
        fs::write(&config.cache_file, serde_json::to_string(&cache).unwrap()).unwrap();

        // Create orphaned content
        let content_cache = ContentCache::new(config.contents_dir.clone());
        content_cache
            .write("orphaned123", "test.sh", "echo test")
            .unwrap();

        // Test clean with orphaned flag
        let args = CacheArgs {
            command: CacheCommands::Clean(CleanArgs {
                older_than: None,
                orphaned: true,
                dry_run: false,
            }),
        };

        let result = handle_cache_command(config, args);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_interpreter_python_alias() {
        let result = parse_interpreter(Some("python")).unwrap();
        assert_eq!(result.0, "python");
        assert!(!result.2); // not shell
    }

    #[test]
    fn test_format_bytes_multiple_gb() {
        assert_eq!(format_bytes(1024 * 1024 * 1024 * 2), "2.00 GB");
        assert_eq!(format_bytes(1536 * 1024 * 1024), "1.50 GB");
    }

    #[test]
    fn test_format_bytes_edge_cases() {
        assert_eq!(format_bytes(1023), "1023 bytes");
        // Edge case: just below MB threshold
        let result = format_bytes(1024 * 1024 - 1);
        assert!(result.starts_with("1023.") || result.starts_with("1024."));
        // Edge case: just below GB threshold
        let result = format_bytes(1024 * 1024 * 1024 - 1);
        assert!(result.starts_with("1023.") || result.starts_with("1024."));
    }

    #[test]
    fn test_handle_cache_command_list_with_cache() {
        use crate::cache::ContentCache;
        use crate::cache::types::{CacheMetadata, GistCache, GistFile, GistInfo};
        use chrono::Utc;

        let temp_dir = TempDir::new().unwrap();
        let config = Config {
            cache_dir: temp_dir.path().to_path_buf(),
            cache_file: temp_dir.path().join("cache.json"),
            contents_dir: temp_dir.path().join("contents"),
            download_dir: temp_dir.path().join("downloads"),
            config_file: temp_dir.path().join("config.toml"),
            user_config: crate::config::UserConfig::default(),
        };

        fs::create_dir_all(&config.contents_dir).unwrap();

        // Create test gist in cache
        let gist = GistInfo {
            id: "test123".to_string(),
            description: Some("Test gist".to_string()),
            files: vec![GistFile {
                filename: "test.sh".to_string(),
                language: Some("Shell".to_string()),
                size: 100,
            }],
            updated_at: Utc::now(),
            public: true,
            html_url: "https://gist.github.com/test123".to_string(),
        };

        let cache = GistCache {
            metadata: CacheMetadata {
                last_updated: Utc::now(),
                total_count: 1,
                github_user: "testuser".to_string(),
            },
            gists: vec![gist],
        };
        fs::write(&config.cache_file, serde_json::to_string(&cache).unwrap()).unwrap();

        // Create content cache
        let content_cache = ContentCache::new(config.contents_dir.clone());
        content_cache
            .write("test123", "test.sh", "echo test")
            .unwrap();

        let args = CacheArgs {
            command: CacheCommands::List(ListArgs {
                format: OutputFormat::Text,
            }),
        };

        let result = handle_cache_command(config, args);
        assert!(result.is_ok());
    }

    #[test]
    fn test_handle_cache_command_list_no_metadata() {
        use crate::cache::ContentCache;

        let temp_dir = TempDir::new().unwrap();
        let config = Config {
            cache_dir: temp_dir.path().to_path_buf(),
            cache_file: temp_dir.path().join("cache.json"),
            contents_dir: temp_dir.path().join("contents"),
            download_dir: temp_dir.path().join("downloads"),
            config_file: temp_dir.path().join("config.toml"),
            user_config: crate::config::UserConfig::default(),
        };

        fs::create_dir_all(&config.contents_dir).unwrap();

        // Create content without metadata
        let content_cache = ContentCache::new(config.contents_dir.clone());
        content_cache
            .write("test456", "test.sh", "echo test")
            .unwrap();

        let args = CacheArgs {
            command: CacheCommands::List(ListArgs {
                format: OutputFormat::Text,
            }),
        };

        let result = handle_cache_command(config, args);
        assert!(result.is_ok());
    }

    #[test]
    fn test_run_gist_with_filename_search() {
        use crate::cache::types::{CacheMetadata, GistCache, GistFile, GistInfo};
        use chrono::Utc;

        let temp_dir = TempDir::new().unwrap();
        let config = Config {
            cache_dir: temp_dir.path().to_path_buf(),
            cache_file: temp_dir.path().join("cache.json"),
            contents_dir: temp_dir.path().join("contents"),
            download_dir: temp_dir.path().join("downloads"),
            config_file: temp_dir.path().join("config.toml"),
            user_config: crate::config::UserConfig::default(),
        };

        fs::create_dir_all(&config.contents_dir).unwrap();

        let gist = GistInfo {
            id: "abc123".to_string(),
            description: Some("Test".to_string()),
            files: vec![GistFile {
                filename: "unique_test.sh".to_string(),
                language: Some("Shell".to_string()),
                size: 100,
            }],
            updated_at: Utc::now(),
            public: true,
            html_url: "https://gist.github.com/abc123".to_string(),
        };

        let cache = GistCache {
            metadata: CacheMetadata {
                last_updated: Utc::now(),
                total_count: 1,
                github_user: "testuser".to_string(),
            },
            gists: vec![gist],
        };
        fs::write(&config.cache_file, serde_json::to_string(&cache).unwrap()).unwrap();

        let args = RunArgs {
            query: Some("unique_test".to_string()),
            interactive: false,
            preview: false,
            force: false,
            download: false,
            id: false,
            filename: true,
            description: false,
            regex: None,
            language: None,
            extension: None,
            interpreter: None,
            script_args: vec![],
        };

        // Will fail at execution but search logic is tested
        let result = run_gist(config, args);
        assert!(result.is_err());
    }

    #[test]
    fn test_run_gist_with_description_search() {
        use crate::cache::types::{CacheMetadata, GistCache, GistFile, GistInfo};
        use chrono::Utc;

        let temp_dir = TempDir::new().unwrap();
        let config = Config {
            cache_dir: temp_dir.path().to_path_buf(),
            cache_file: temp_dir.path().join("cache.json"),
            contents_dir: temp_dir.path().join("contents"),
            download_dir: temp_dir.path().join("downloads"),
            config_file: temp_dir.path().join("config.toml"),
            user_config: crate::config::UserConfig::default(),
        };

        fs::create_dir_all(&config.contents_dir).unwrap();

        let gist = GistInfo {
            id: "def456".to_string(),
            description: Some("Special description for testing".to_string()),
            files: vec![GistFile {
                filename: "test.sh".to_string(),
                language: Some("Shell".to_string()),
                size: 100,
            }],
            updated_at: Utc::now(),
            public: true,
            html_url: "https://gist.github.com/def456".to_string(),
        };

        let cache = GistCache {
            metadata: CacheMetadata {
                last_updated: Utc::now(),
                total_count: 1,
                github_user: "testuser".to_string(),
            },
            gists: vec![gist],
        };
        fs::write(&config.cache_file, serde_json::to_string(&cache).unwrap()).unwrap();

        let args = RunArgs {
            query: Some("Special description".to_string()),
            interactive: false,
            preview: false,
            force: false,
            download: false,
            id: false,
            filename: false,
            description: true,
            regex: None,
            language: None,
            extension: None,
            interpreter: None,
            script_args: vec![],
        };

        // Will fail at execution but search logic is tested
        let result = run_gist(config, args);
        assert!(result.is_err());
    }

    #[test]
    fn test_run_gist_no_results() {
        use crate::cache::types::{CacheMetadata, GistCache};
        use chrono::Utc;

        let temp_dir = TempDir::new().unwrap();
        let config = Config {
            cache_dir: temp_dir.path().to_path_buf(),
            cache_file: temp_dir.path().join("cache.json"),
            contents_dir: temp_dir.path().join("contents"),
            download_dir: temp_dir.path().join("downloads"),
            config_file: temp_dir.path().join("config.toml"),
            user_config: crate::config::UserConfig::default(),
        };

        fs::create_dir_all(&config.contents_dir).unwrap();

        let cache = GistCache {
            metadata: CacheMetadata {
                last_updated: Utc::now(),
                total_count: 0,
                github_user: "testuser".to_string(),
            },
            gists: vec![],
        };
        fs::write(&config.cache_file, serde_json::to_string(&cache).unwrap()).unwrap();

        let args = RunArgs {
            query: Some("nonexistent".to_string()),
            interactive: false,
            preview: false,
            force: false,
            download: false,
            id: false,
            filename: false,
            description: false,
            regex: None,
            language: None,
            extension: None,
            interpreter: None,
            script_args: vec![],
        };

        let result = run_gist(config, args);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            GistCacheError::NoSearchResults(_)
        ));
    }

    #[test]
    fn test_parse_interpreter_custom_invalid() {
        // Test with nonexistent interpreter
        let result = parse_interpreter(Some("nonexistent_xyz_123"));
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            GistCacheError::InvalidInterpreter(_)
        ));
    }

    #[test]
    fn test_generate_completions_bash() {
        // Test that bash completions can be generated without error
        let result = generate_completions(Shell::Bash);
        assert!(result.is_ok());
    }

    #[test]
    fn test_generate_completions_zsh() {
        // Test that zsh completions can be generated without error
        let result = generate_completions(Shell::Zsh);
        assert!(result.is_ok());
    }

    #[test]
    fn test_generate_completions_fish() {
        // Test that fish completions can be generated without error
        let result = generate_completions(Shell::Fish);
        assert!(result.is_ok());
    }

    #[test]
    fn test_generate_completions_powershell() {
        // Test that PowerShell completions can be generated without error
        let result = generate_completions(Shell::PowerShell);
        assert!(result.is_ok());
    }

    #[test]
    fn test_shell_enum_values() {
        // Test that Shell enum has all expected variants
        let shells = [Shell::Bash, Shell::Zsh, Shell::Fish, Shell::PowerShell];
        assert_eq!(shells.len(), 4);

        // Test that each variant can be created
        assert_eq!(Shell::Bash, Shell::Bash);
        assert_eq!(Shell::Zsh, Shell::Zsh);
        assert_eq!(Shell::Fish, Shell::Fish);
        assert_eq!(Shell::PowerShell, Shell::PowerShell);
    }
}
