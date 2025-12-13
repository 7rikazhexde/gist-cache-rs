# gist-cache-rs

[![Deploy Docs](https://github.com/7rikazhexde/gist-cache-rs/actions/workflows/deploy-mdbook.yml/badge.svg)](https://7rikazhexde.github.io/gist-cache-rs/)
[![Crates.io](https://img.shields.io/crates/v/gist-cache-rs.svg)](https://crates.io/crates/gist-cache-rs)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A high-performance CLI tool written in Rust for efficiently caching, searching, and executing GitHub Gists.

## Features

- ⚡ **High Speed**: Lightning-fast caching and search operations
- 🔄 **Smart Updates**: Incremental cache updates that only fetch what's changed
- 💾 **2-Layer Caching**: Intelligent caching for 20x faster execution
- 🔍 **Flexible Search**: Search by ID, filename, or description
- ▶️ **Multi-Language Support**: bash, python, ruby, node, php, perl, pwsh, TypeScript, and more
- 💬 **Interactive Mode**: Full support for interactive scripts
- 📦 **Modern Python**: uv support with PEP 723 metadata compatibility
- 📥 **Easy Downloads**: Save Gist files to your download folder
- 🗂️ **Cache Management**: Powerful cache inspection and maintenance

**Supported Platforms**: Linux, macOS, Windows 10 or later

## Quick Start

### Prerequisites

- Rust toolchain (1.85 or later)
- GitHub CLI (`gh`) - Authenticated with `gh auth login`

### Installation

**Using Setup Script (Recommended):**

```bash
# Linux / macOS
git clone https://github.com/7rikazhexde/gist-cache-rs.git
cd gist-cache-rs
./script/setup.sh install

# Windows
git clone https://github.com/7rikazhexde/gist-cache-rs.git
cd gist-cache-rs
.\script\setup.ps1 install
```

The setup script performs interactive prerequisite checks, builds, installs, and creates the initial cache.

**Using Cargo:**

```bash
cargo install gist-cache-rs
```

For more installation options, see the [Installation Guide](https://7rikazhexde.github.io/gist-cache-rs/user-guide/installation.html).

### Basic Usage

```bash
# Create initial cache
gist-cache-rs update

# Search and execute a Gist
gist-cache-rs run backup bash

# Execute Python script with arguments
gist-cache-rs run data_analysis.py python3 input.csv

# Preview without execution
gist-cache-rs run --preview backup

# Interactive mode
gist-cache-rs run --interactive setup-wizard

# Download Gist to ~/Downloads
gist-cache-rs run --download backup bash

# Update cache and execute latest version
gist-cache-rs run --force backup bash
```

## Shell Completions

`gist-cache-rs` supports shell completions for Bash, Zsh, Fish, and PowerShell. This enables auto-completion of commands, subcommands, and options by pressing the Tab key.

### Generate Completion Scripts

```bash
# Bash
gist-cache-rs completions bash > ~/.local/share/bash-completion/completions/gist-cache-rs

# Zsh
gist-cache-rs completions zsh > ~/.zfunc/_gist-cache-rs

# Fish
gist-cache-rs completions fish > ~/.config/fish/completions/gist-cache-rs.fish

# PowerShell
gist-cache-rs completions powershell > ~\Documents\PowerShell\Scripts\gist-cache-rs.ps1
```

### Shell-Specific Setup

**Bash:**

```bash
# Add to ~/.bashrc if needed
source ~/.local/share/bash-completion/completions/gist-cache-rs
```

**Zsh:**

```bash
# Add to ~/.zshrc if ~/.zfunc is not in fpath
fpath=(~/.zfunc $fpath)
autoload -Uz compinit && compinit
```

**Fish:**

```bash
# Fish automatically loads completions from ~/.config/fish/completions/
# No additional configuration needed
```

**PowerShell:**

```powershell
# Add to your PowerShell profile
. ~\Documents\PowerShell\Scripts\gist-cache-rs.ps1
```

## Updating the Tool

To update `gist-cache-rs` to the latest version, use `cargo install`:

```bash
cargo install gist-cache-rs
```

For local development builds, use:

```bash
cargo install --path .
```

## Documentation

**📖 Full Documentation**: [https://7rikazhexde.github.io/gist-cache-rs/](https://7rikazhexde.github.io/gist-cache-rs/)

### For Users

- [Installation Guide](https://7rikazhexde.github.io/gist-cache-rs/user-guide/installation.html)
- [Quick Start](https://7rikazhexde.github.io/gist-cache-rs/user-guide/quickstart.html)
- [Usage Examples](https://7rikazhexde.github.io/gist-cache-rs/user-guide/examples.html)

### For Developers

- [Architecture & Design](https://7rikazhexde.github.io/gist-cache-rs/developer-guide/architecture.html)
- [Testing Strategy](https://7rikazhexde.github.io/gist-cache-rs/developer-guide/testing.html)
- [Coverage Analysis](https://7rikazhexde.github.io/gist-cache-rs/developer-guide/coverage.html)
- [Test Inventory](https://7rikazhexde.github.io/gist-cache-rs/developer-guide/test-inventory.html)

## Cache Management

```bash
# List cached Gists
gist-cache-rs cache list

# Check cache size
gist-cache-rs cache size

# Clean old cache entries
gist-cache-rs cache clean --older-than 30        # Remove entries older than 30 days
gist-cache-rs cache clean --orphaned             # Remove orphaned cache files
gist-cache-rs cache clean --dry-run --orphaned   # Preview what would be deleted

# Clear all caches
gist-cache-rs cache clear
```

## Development

### Build from Source

```bash
git clone https://github.com/7rikazhexde/gist-cache-rs.git
cd gist-cache-rs
cargo build --release
cargo install --path .
```

### Run Tests

```bash
cargo test
```

### Code Quality

```bash
# Run all checks (format, lint, test)
just check

# Format code
just fmt

# Lint with clippy
just lint
```

## Uninstallation

**Linux / macOS:**

```bash
./script/setup.sh uninstall
# Or manually:
# cargo uninstall gist-cache-rs
# rm -rf ~/.cache/gist-cache/
```

**Windows:**

```powershell
.\script\setup.ps1 uninstall
# Or manually:
# cargo uninstall gist-cache-rs
# Remove-Item -Recurse -Force "$env:LOCALAPPDATA\gist-cache"
```

## Contributing

Contributions are welcome! Please see the [Architecture Guide](https://7rikazhexde.github.io/gist-cache-rs/developer-guide/architecture.html) for project structure and design patterns.

## License

MIT License - see LICENSE file for details

## Links

- [Documentation](https://7rikazhexde.github.io/gist-cache-rs/)
- [Repository](https://github.com/7rikazhexde/gist-cache-rs)
- [Issues](https://github.com/7rikazhexde/gist-cache-rs/issues)
- [Crates.io](https://crates.io/crates/gist-cache-rs)
