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
- 📊 **Progress Display**: Visual feedback with progress bars and spinners
- 🎯 **Interactive Selection**: Intuitive arrow-key navigation for selecting Gists
- 📋 **Output Format Options**: JSON output for scripting and automation
- ⚙️ **Flexible Configuration**: CLI and file-based configuration for defaults and preferences

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
# Create initial cache (with progress display)
gist-cache-rs update

# Update cache with verbose output
gist-cache-rs update --verbose

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

`gist-cache-rs` supports shell completions for Bash, Zsh, Fish, and PowerShell.

```bash
# Generate completion script for your shell
gist-cache-rs completions <SHELL>

# Example for Bash
gist-cache-rs completions bash > ~/.local/share/bash-completion/completions/gist-cache-rs
```

For detailed installation instructions, backup procedures, and shell-specific setup, see the [Shell Completions Guide](https://7rikazhexde.github.io/gist-cache-rs/user-guide/shell-completions.html).

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
# List cached Gists (human-readable format)
gist-cache-rs cache list

# List cached Gists in JSON format (for scripting)
gist-cache-rs cache list --format json

# Filter with jq (requires jq installed)
gist-cache-rs cache list --format json | jq '.[] | select(.description | contains("backup"))'

# Check cache size
gist-cache-rs cache size

# Clean old cache entries
gist-cache-rs cache clean --older-than 30        # Remove entries older than 30 days
gist-cache-rs cache clean --orphaned             # Remove orphaned cache files
gist-cache-rs cache clean --dry-run --orphaned   # Preview what would be deleted

# Clear all caches
gist-cache-rs cache clear
```

## Configuration

Customize default behavior with the config command:

```bash
# Set default interpreter
gist-cache-rs config set defaults.interpreter python3

# Enable execution confirmation (for safety)
gist-cache-rs config set execution.confirm_before_run true

# Set cache retention period
gist-cache-rs config set cache.retention_days 30

# View current configuration
gist-cache-rs config show

# Get specific value
gist-cache-rs config get defaults.interpreter

# Edit config file directly
gist-cache-rs config edit

# Reset to defaults
gist-cache-rs config reset
```

For detailed configuration options, see the [Configuration Guide](https://7rikazhexde.github.io/gist-cache-rs/user-guide/configuration.html).

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
