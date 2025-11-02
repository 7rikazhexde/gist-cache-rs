use clap::{Args, Parser, Subcommand};
use colored::Colorize;
use gist_cache_rs::*;
use std::fs;

#[derive(Parser)]
#[command(name = "gist-cache-rs")]
#[command(about = "Gistキャッシュ・実行システム (Rust実装版)", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// キャッシュを更新
    Update(UpdateArgs),
    /// キャッシュから検索して実行
    Run(RunArgs),
    /// キャッシュ管理
    Cache(CacheArgs),
}

#[derive(Args)]
struct UpdateArgs {
    /// 強制的に全件更新
    #[arg(short, long)]
    force: bool,

    /// 詳細な進捗情報を表示
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Args)]
struct RunArgs {
    /// 検索キーワード (ID、ファイル名、または説明文)
    query: Option<String>,

    /// 対話的スクリプト実行モード
    #[arg(short, long)]
    interactive: bool,

    /// プレビューモード（内容表示のみ）
    #[arg(short, long)]
    preview: bool,

    /// 実行前にGistキャッシュを更新
    #[arg(short, long)]
    force: bool,

    /// ID直接指定モード
    #[arg(long)]
    id: bool,

    /// ファイル名で検索
    #[arg(long)]
    filename: bool,

    /// 説明文で検索
    #[arg(long)]
    description: bool,

    /// インタープリタまたは実行コマンド (bash, python3, uv, など)
    #[arg(value_name = "INTERPRETER")]
    interpreter: Option<String>,

    /// スクリプトに渡す追加の引数
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    script_args: Vec<String>,
}

#[derive(Args)]
struct CacheArgs {
    #[command(subcommand)]
    command: CacheCommands,
}

#[derive(Subcommand)]
enum CacheCommands {
    /// キャッシュされたGistの一覧を表示
    List,
    /// キャッシュの合計サイズを表示
    Size,
    /// 古いキャッシュを削除（未実装）
    Clean,
    /// 全てのキャッシュを削除
    Clear,
}

fn main() {
    if let Err(e) = run() {
        eprintln!("{} {}", "エラー:".red().bold(), e);
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
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
                updater.update(false)?;  // 差分更新（force=false）
            }
            
            run_gist(config, args)?;
        }
        Commands::Cache(args) => {
            handle_cache_command(config, args)?;
        }
    }

    Ok(())
}

fn print_run_help() {
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
    println!();
    println!("{}", "引数指定を確認してください:".red().bold());
    println!("  ✅ uv例: gist-cache-rs run --description numpy uv input.csv");
    println!();
    println!("For more information, try '--help'");
}

fn run_gist(config: Config, args: RunArgs) -> Result<()> {
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
    let runner = ScriptRunner::new(
        gist.clone(),
        interpreter,
        run_command,
        is_shell,
        args.interactive,
        args.preview,
        force_file_based,
        args.script_args,
        config,
    );

    runner.run()?;

    Ok(())
}

fn parse_interpreter(interpreter: Option<&str>) -> Result<(String, Option<String>, bool, bool)> {
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
            Ok(("deno".to_string(), Some("deno run".to_string()), false, true))
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
            if let Ok(output) = std::process::Command::new("which").arg(custom).output() {
                if !output.status.success() {
                    eprintln!();
                    eprintln!("{}", "サポートされているインタープリタ:".green());
                    eprintln!("  bash, sh, zsh, python3, python, uv, ruby, node, perl, php, pwsh, powershell");
                    eprintln!("  ts-node, deno, bun (TypeScript)");
                    eprintln!();
                    eprintln!("{}", "引数指定を確認してください:".yellow());
                    eprintln!("  ✅ uv例: gist-cache-rs run --description numpy uv input.csv");
                    eprintln!();
                    return Err(GistCacheError::InvalidInterpreter(custom.to_string()));
                }
            }
            Ok((custom.to_string(), None, false, false))
        }
    }
}

fn handle_cache_command(config: Config, args: CacheArgs) -> Result<()> {
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
                        // 修正: as_deref() を使用
                        let desc = gist
                            .description
                            .as_deref()
                            .unwrap_or("No description");
                        
                        let files: Vec<_> = gist.files.iter().map(|f| f.filename.as_str()).collect();

                        println!("{}", format!("ID: {}", gist.id).green());
                        println!("  説明: {}", desc);
                        println!("  ファイル: {}", files.join(", "));
                        println!("  更新日時: {}", gist.updated_at.format("%Y-%m-%d %H:%M:%S"));
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
                println!(
                    "{}",
                    format!("合計: {}件", gist_ids.len()).cyan().bold()
                );
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
            println!("{}", format!("合計サイズ: {}", format_bytes(total_size)).green());
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
                format!("{}件のGistキャッシュを削除します。よろしいですか？", gist_count)
                    .yellow()
            );
            println!("  {}", "この操作は取り消せません。".red());
            println!();
            print!("{}", "続行しますか？ (y/N): ");

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

fn format_bytes(bytes: u64) -> String {
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
