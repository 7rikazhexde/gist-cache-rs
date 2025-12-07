use colored::Colorize;
use gist_cache_rs::run_cli;

fn main() {
    if let Err(e) = run_cli() {
        eprintln!("{} {}", "Error:".red().bold(), e);
        std::process::exit(1);
    }
}
