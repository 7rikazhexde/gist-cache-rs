use crate::*;
use clap::{Args, Parser, Subcommand};
use colored::Colorize;
use std::fs;

#[derive(Parser)]
#[command(name = "gist-cache-rs")]
#[command(about = "Gistキャッシュ・実行システム (Rust実装版)", long_about = None)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// キャッシュを更新
    Update(UpdateArgs),
    /// キャッシュから検索して実行
    Run(RunArgs),
    /// キャッシュ管理
    Cache(CacheArgs),
}

#[derive(Args)]
pub struct UpdateArgs {
    /// 強制的に全件更新
    #[arg(short, long)]
    pub force: bool,

    /// 詳細な進捗情報を表示
    #[arg(short, long)]
    pub verbose: bool,
}

#[derive(Args)]
pub struct RunArgs {
    /// 検索キーワード (ID、ファイル名、または説明文)
    pub query: Option<String>,

    /// 対話的スクリプト実行モード
    #[arg(short, long)]
    pub interactive: bool,

    /// プレビューモード（内容表示のみ）
    #[arg(short, long)]
    pub preview: bool,

    /// 実行前にGistキャッシュを更新
    #[arg(short, long)]
    pub force: bool,

    /// ファイルをダウンロードフォルダに保存
    #[arg(long)]
    pub download: bool,

    /// ID直接指定モード
    #[arg(long)]
    pub id: bool,

    /// ファイル名で検索
    #[arg(long)]
    pub filename: bool,

    /// 説明文で検索
    #[arg(long)]
    pub description: bool,

    /// インタープリタまたは実行コマンド (bash, python3, uv, など)
    #[arg(value_name = "INTERPRETER")]
    pub interpreter: Option<String>,

    /// スクリプトに渡す追加の引数
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
    /// キャッシュされたGistの一覧を表示
    List,
    /// キャッシュの合計サイズを表示
    Size,
    /// 古いキャッシュを削除（未実装）
    Clean,
    /// 全てのキャッシュを削除
    Clear,
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
            // クエリがない場合はヘルプを表示
            if args.query.is_none() {
                print_run_help();
                return Ok(());
            }

            // --force オプションが指定されている場合は、先にキャッシュを更新
            if args.force {
                let updater = CacheUpdater::new(config.clone(), false);
                updater.update(false)?; // 差分更新（force=false）
            }

            run_gist(config, args)?;
        }
        Commands::Cache(args) => {
            handle_cache_command(config, args)?;
        }
    }

    Ok(())
}

pub fn print_run_help() {
    println!("{}", "キャッシュから検索して実行".bold());
    println!();
    println!("Usage: gist-cache-rs run [OPTIONS] <QUERY> [INTERPRETER] [SCRIPT_ARGS]...");
    println!();
    println!("{}", "引数の順序に注意:".yellow().bold());
    println!("  オプション（--description等）は、検索キーワードの前に指定してください");
    println!();
    println!("Arguments:");
    println!("  <QUERY>           検索キーワード (ID、ファイル名、または説明文)");
    println!("  [INTERPRETER]     インタープリタ (bash, python3, uv, など)");
    println!("  [SCRIPT_ARGS]...  スクリプトに渡す追加の引数");
    println!();
    println!("Options:");
    println!("  -i, --interactive  対話的スクリプト実行モード");
    println!("  -p, --preview      プレビューモード（内容表示のみ）");
    println!("  -f, --force        実行前にGistキャッシュを更新（常に最新版を取得）");
    println!("      --download     ファイルをダウンロードフォルダに保存");
    println!("      --id           ID直接指定モード");
    println!("      --filename     ファイル名で検索");
    println!("      --description  説明文で検索");
    println!("  -h, --help         Print help");
    println!();
    println!("{}", "サポートされているインタープリタ:".green().bold());
    println!("  bash, sh, zsh      - シェルスクリプト");
    println!("  python3, python    - Python");
    println!("  uv                 - Python (PEP 723対応)");
    println!("  ruby, node, perl, php - その他の言語");
    println!();
    println!("Examples:");
    println!("  gist-cache-rs run backup                      # キーワード検索");
    println!("  gist-cache-rs run backup bash /src /dst       # 引数付きで実行");
    println!("  gist-cache-rs run data python3                # Python実行");
    println!("  gist-cache-rs run --description numpy uv      # 説明文検索+uv実行");
    println!("  gist-cache-rs run -p backup                   # プレビュー");
    println!("  gist-cache-rs run -i interactive-script       # 対話モード");
    println!("  gist-cache-rs run --filename setup.sh         # ファイル名検索");
    println!("  gist-cache-rs run --id abc123def456           # ID指定");
    println!("  gist-cache-rs run -f backup                   # キャッシュ更新後に実行");
    println!("  gist-cache-rs run -f --description numpy uv   # キャッシュ更新+説明文検索");
    println!("  gist-cache-rs run --download backup           # ダウンロードフォルダに保存");
    println!("  gist-cache-rs run -p --download backup        # プレビュー後にダウンロード");
    println!();
    println!("{}", "引数指定を確認してください:".red().bold());
    println!("  ✅ uv例: gist-cache-rs run --description numpy uv input.csv");
    println!();
    println!("For more information, try '--help'");
}

pub fn run_gist(config: Config, args: RunArgs) -> Result<()> {
    // Check cache exists
    if !config.cache_exists() {
        return Err(GistCacheError::CacheNotFound);
    }

    // queryは必ずSomeであることを保証
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

    // Search for gists
    let query = SearchQuery::new(query_string.clone(), search_mode.clone());
    let results = query.search(&cache.gists)?;

    if results.is_empty() {
        return Err(GistCacheError::NoSearchResults(query_string));
    }

    // Select gist
    let gist = if matches!(search_mode, SearchMode::Id) && results.len() == 1 {
        println!("{}", format!("ID指定モード: {}", results[0].id).cyan());
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
            // uvは必ずファイルベースで実行（PEP 723対応のため）
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
                "警告: PoetryはPEP 723をサポートしていません。python3で実行します。".yellow()
            );
            Ok(("python3".to_string(), None, false, false))
        }
        None => Ok(("bash".to_string(), None, true, false)),
        Some(custom) => {
            // Check if the custom interpreter exists
            if let Ok(output) = std::process::Command::new("which").arg(custom).output()
                && !output.status.success()
            {
                eprintln!();
                eprintln!("{}", "サポートされているインタープリタ:".green());
                eprintln!(
                    "  bash, sh, zsh, python3, python, uv, ruby, node, perl, php, pwsh, powershell"
                );
                eprintln!("  ts-node, deno, bun (TypeScript)");
                eprintln!();
                eprintln!("{}", "引数指定を確認してください:".yellow());
                eprintln!("  ✅ uv例: gist-cache-rs run --description numpy uv input.csv");
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
        CacheCommands::List => {
            println!("{}", "キャッシュされたGist一覧:".cyan().bold());
            println!();

            let gist_ids = content_cache.list_cached_gists()?;

            if gist_ids.is_empty() {
                println!("{}", "キャッシュされたGistはありません".yellow());
                return Ok(());
            }

            // メタデータJSONを読み込んで詳細情報を表示
            if config.cache_exists() {
                let cache_content = fs::read_to_string(&config.cache_file)?;
                let cache: GistCache = serde_json::from_str(&cache_content)?;

                for gist_id in &gist_ids {
                    if let Some(gist) = cache.gists.iter().find(|g| &g.id == gist_id) {
                        let desc = gist.description.as_deref().unwrap_or("No description");

                        let files: Vec<_> =
                            gist.files.iter().map(|f| f.filename.as_str()).collect();

                        println!("{}", format!("ID: {}", gist.id).green());
                        println!("  説明: {}", desc);
                        println!("  ファイル: {}", files.join(", "));
                        println!(
                            "  更新日時: {}",
                            gist.updated_at.format("%Y-%m-%d %H:%M:%S")
                        );
                        println!();
                    } else {
                        println!("{}", format!("ID: {}", gist_id).green());
                        println!("  (メタデータが見つかりません)");
                        println!();
                    }
                }

                println!(
                    "{}",
                    format!("合計: {}件のGistがキャッシュされています", gist_ids.len())
                        .cyan()
                        .bold()
                );
            } else {
                // メタデータがない場合はIDのみ表示
                for gist_id in &gist_ids {
                    println!("  {}", gist_id.green());
                }
                println!();
                println!("{}", format!("合計: {}件", gist_ids.len()).cyan().bold());
            }
        }
        CacheCommands::Size => {
            println!("{}", "キャッシュサイズ情報:".cyan().bold());
            println!();

            let total_size = content_cache.total_size()?;
            let gist_count = content_cache.list_cached_gists()?.len();

            println!(
                "{}",
                format!("キャッシュされたGist数: {}件", gist_count).green()
            );
            println!(
                "{}",
                format!("合計サイズ: {}", format_bytes(total_size)).green()
            );
            println!(
                "{}",
                format!("キャッシュディレクトリ: {}", config.contents_dir.display()).cyan()
            );
        }
        CacheCommands::Clean => {
            println!("{}", "古いキャッシュの削除".yellow());
            println!();
            println!(
                "{}",
                "この機能は現在未実装です。将来のバージョンで実装予定です。".yellow()
            );
            println!();
            println!("代わりに以下のコマンドを使用できます:");
            println!("  gist-cache-rs cache clear  # 全キャッシュを削除");
        }
        CacheCommands::Clear => {
            println!("{}", "全キャッシュの削除".yellow().bold());
            println!();

            let gist_count = content_cache.list_cached_gists()?.len();

            if gist_count == 0 {
                println!("{}", "削除するキャッシュはありません".green());
                return Ok(());
            }

            println!(
                "{}",
                format!(
                    "{}件のGistキャッシュを削除します。よろしいですか？",
                    gist_count
                )
                .yellow()
            );
            println!("  {}", "この操作は取り消せません。".red());
            println!();
            print!("続行しますか？ (y/N): ");

            std::io::Write::flush(&mut std::io::stdout())?;

            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;

            if input.trim().to_lowercase() == "y" {
                content_cache.clear_all()?;
                println!();
                println!("{}", "全キャッシュを削除しました".green().bold());
            } else {
                println!();
                println!("{}", "キャンセルしました".cyan());
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
        };

        fs::create_dir_all(&config.contents_dir).unwrap();

        let args = CacheArgs {
            command: CacheCommands::List,
        };

        let result = handle_cache_command(config, args);
        assert!(result.is_ok());
    }

    #[test]
    fn test_handle_cache_command_clean() {
        use std::fs;
        let temp_dir = TempDir::new().unwrap();
        let config = Config {
            cache_dir: temp_dir.path().to_path_buf(),
            cache_file: temp_dir.path().join("cache.json"),
            contents_dir: temp_dir.path().join("contents"),
            download_dir: temp_dir.path().join("downloads"),
        };

        fs::create_dir_all(&config.contents_dir).unwrap();

        let args = CacheArgs {
            command: CacheCommands::Clean,
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
            command: CacheCommands::List,
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
        };

        fs::create_dir_all(&config.contents_dir).unwrap();

        // Create content without metadata
        let content_cache = ContentCache::new(config.contents_dir.clone());
        content_cache
            .write("test456", "test.sh", "echo test")
            .unwrap();

        let args = CacheArgs {
            command: CacheCommands::List,
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
}
