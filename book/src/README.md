# gist-cache-rs

A CLI tool (Rust implementation) for efficiently caching, searching, and executing GitHub Gists.

## Features

- ⚡ **High Speed**: Fast caching operations and searches implemented in Rust
- 🔄 **Incremental Updates**: Supports efficient differential cache updates
- 💾 **2-Layer Caching**: Caches both metadata and content for accelerated execution
- 🔍 **Diverse Search**: Search by ID, filename, or description
- ▶️ **Execution Support**: Supports multiple interpreters (bash, python, ruby, node, php, perl, pwsh, TypeScript)
- 💬 **Interactive Mode**: Interactive execution of scripts using commands like `read`
- 📦 **uv Support**: Execution compatible with PEP 723 metadata
- 📥 **Download Functionality**: Save Gist files to the download folder
- 🗂️ **Cache Management**: Efficient operation with powerful cache management commands

This project supports Linux, macOS, and Windows (Windows 10 or later).

## 📋 Prerequisites

- Rust toolchain (1.85 or later)
- GitHub CLI (`gh`) - Authenticated

## 🔧 Installation

**Setup Script (Recommended):**

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

Performs interactive prerequisite checks, build, installation, and initial cache creation.

**Other Installation Methods:** Refer to [INSTALL.md](docs/INSTALL.md).

## 🚀 [Quick Start](docs/QUICKSTART.md)

[EXAMPLES.md](docs/EXAMPLES.md) also summarizes practical usage examples.

```bash
# Initial cache creation
gist-cache-rs update

# Search and execute Gist
gist-cache-rs run backup bash

# Execute Python script
gist-cache-rs run data_analysis.py python3 input.csv
```

## 🔄 Cache Updates

```bash
# Incremental update (default)
gist-cache-rs update

# With verbose output
gist-cache-rs update --verbose

# Force full update
gist-cache-rs update --force
```

## 🔃 Application Update

You can update gist-cache-rs itself to the latest version:

```bash
# Update to the latest version
gist-cache-rs self update

# Check for updates only
gist-cache-rs self update --check

# Update by building from source
gist-cache-rs self update --from-source
```

For details, refer to [SELF-UPDATE.md](docs/SELF-UPDATE.md).

## 💾 Cache Mechanism

gist-cache-rs has a 2-layer cache structure:

### Metadata Cache

- **Content**: Metadata such as Gist ID, filename, description, and update time
- **Update**: Incremental or full updates with the `update` command

### Content Cache

- **Content**: The actual script body
- **Update**: Automatically created on first execution, automatically deleted when Gist updates are detected
- **Advantage**: Subsequent executions are approximately 20 times faster (no network access required)

### Cache Location

**Linux / macOS:**

```text
~/.cache/gist-cache/
├── cache.json                    # Metadata cache
└── contents/                     # Content cache
    ├── {gist_id_1}/
    │   └── {filename_1}
    ├── {gist_id_2}/
    │   └── {filename_2}
    └── ...
```

**Windows:**

```text
%LOCALAPPDATA%\gist-cache\
├── cache.json                    # Metadata cache
└── contents\                     # Content cache
    ├── {gist_id_1}\
    │   └── {filename_1}
    ├── {gist_id_2}\
    │   └── {filename_2}
    └── ...
```

## 🔍 Gist Search and Execution

### Search Methods

```bash
# Keyword search (filename or description)
gist-cache-rs run backup

# Direct ID specification
gist-cache-rs run abc123def456789

# Search by filename
gist-cache-rs run --filename setup.sh

# Search by description
gist-cache-rs run --description "data processor"
```

### Interpreter Specification

```bash
# Execute as a Bash script (default)
gist-cache-rs run backup bash

# Execute with Python3
gist-cache-rs run data-analysis python3

# Execute with uv (PEP 723 compatible)
gist-cache-rs run ml-script uv

# Other interpreters
gist-cache-rs run script ruby
gist-cache-rs run script node
gist-cache-rs run script.ts deno     # TypeScript (Deno)
# ... also supports ruby, perl, php, pwsh, ts-node, bun
```

### Passing Arguments

```bash
# Pass arguments to the script
gist-cache-rs run backup bash /src /dst

# Arguments to Python script
gist-cache-rs run data_analysis.py python3 input.csv --output result.json

# Pass arguments when executing with uv
gist-cache-rs run ml-training uv --epochs 100 --batch-size 32
```

### Interactive Mode

```bash
# Execute an interactive script (when using `read` command, etc.)
gist-cache-rs run --interactive create-folders

# Shorthand
gist-cache-rs run -i config-tool bash
```

### Preview

You can check the content of a script without executing it:

```bash
# Display content without execution
gist-cache-rs run --preview backup

# Shorthand
gist-cache-rs run -p data-analysis

# Combine with direct ID specification
gist-cache-rs run -p --id abc123def456

# Combine with filename search
gist-cache-rs run -p --filename setup.sh
```

**Preview Display Content**:

- Description
- Files
- Full script content (without syntax highlighting)

**Usage**:

- Check script content before execution
- Confirm arguments and settings
- Prevent accidental execution of wrong scripts

### File Download

You can save Gist files to the download folder (`~/Downloads`):

```bash
# Download after execution
gist-cache-rs run --download backup bash

# Download after preview
gist-cache-rs run --preview --download script.py

# Download by ID
gist-cache-rs run --download --id abc123def456
```

**Features**:

- Saves to the download folder (`~/Downloads`)
- Convenient when you want to save files separately from executable scripts
- Cache is automatically created during download, speeding up subsequent executions
- Can be used with other options (`--preview`, `--force`, `--interactive`, etc.)

**Download Operation Order**:

1. `--preview --download`: Preview display → Download
2. `--force --download`: Cache update → Execute → Download
3. `--download` only: Execute → Download

### Force Update Option

```bash
# Fetch the latest Gist information before execution
# Automatically re-fetches if content cache is updated
gist-cache-rs run --force backup bash

# Combine with description search
gist-cache-rs run --force --description "data processor" python3
```

## ⌨️ Alias Configuration

For more convenient use, you can set up your preferred aliases:

### Automatic Configuration (when using setup.sh)

Interactive setup during installation:

- Recommended aliases (`gcrsu`, `gcrsr`)
- Custom alias names

### Manual Configuration

```bash
# Add to ~/.bashrc or ~/.zshrc
alias gcrsu='gist-cache-rs update'
alias gcrsr='gist-cache-rs run'

# Apply changes
source ~/.bashrc
```

Example Usage:

```bash
gcrsu  # Cache update
gcrsr backup bash /src /dst  # Gist execution
gcrsr -p script  # Preview
gcrsr -i interactive-script  # Interactive mode
gcrsr --download backup  # Download
gcrsr -p --download script  # Download after preview
```

## 🗑️ Uninstallation

### Linux / macOS

```bash
# Automatic uninstallation
./script/setup.sh uninstall

# Manual uninstallation
cargo uninstall gist-cache-rs
rm -rf ~/.cache/gist-cache/
```

### Windows

```powershell
# Automatic uninstallation
.\script\setup.ps1 uninstall

# Manual uninstallation
cargo uninstall gist-cache-rs
Remove-Item -Recurse -Force "$env:LOCALAPPDATA\gist-cache"
```

## ❓ Help

```bash
# Overall help
gist-cache-rs --help

# Subcommand help
gist-cache-rs update --help
gist-cache-rs run --help
```

## 🔍 Troubleshooting

### Error: Cache file not found. Please run 'gist-cache-rs update' first

**Solution:** Run `gist-cache-rs update` to create the cache.

### Error: GitHub CLI (gh) is not authenticated

**Solution:** Run `gh auth login` to authenticate.

### Warning: Rate limit remaining is low (50)

**Solution:** Wait for a while and retry, or avoid forced updates.

### command not found: gist-cache-rs

**Solution:**

- Check if `~/.cargo/bin` is included in your PATH
- Or copy the binary to `/usr/local/bin`

For details, refer to [INSTALL.md](docs/INSTALL.md).

## 📁 Project Structure

```bash
gist-cache-rs/
├── Cargo.toml           # Project configuration
├── src/
│   ├── main.rs          # CLI entry point
│   ├── lib.rs           # Library root
│   ├── error.rs         # Error type definition
│   ├── config.rs        # Configuration management
│   ├── cache/           # Cache module
│   ├── github/          # GitHub API module
│   ├── search/          # Search module
│   └── execution/       # Execution module
├── script/
│   ├── setup.sh         # Setup script (Linux/macOS)
│   └── setup.ps1        # Setup script (Windows)
└── README.md
```

## 🗂️ Cache Management

You can efficiently manage the content cache of executed Gists:

```bash
# Display cache list
gist-cache-rs cache list

# Check cache size
gist-cache-rs cache size

# Delete orphaned cache (not yet implemented)
gist-cache-rs cache clean

# Clear all caches
gist-cache-rs cache clear
```

### Cache Behavior

1. **First Execution**: Fetches content from GitHub API and creates a cache after execution
2. **Subsequent Executions**: Reads from cache for faster execution (approx. 20 times faster)
3. **Gist Update**: `update` command detects updates and automatically deletes the cache
4. **First Execution After Update**: Fetches the latest version from API and creates a new cache

## 🛠️ Development Environment Setup

If you contribute to this project, the following tools will improve your development experience.

### Setting up pre-commit hooks

We recommend using [prek](https://github.com/j178/prek) (a fast pre-commit tool written in Rust) to maintain code quality.

**prek Installation:**

```bash
# Install with cargo
cargo install --locked prek

# Or with your system's package manager
# macOS: brew install prek
# Other installation methods: https://github.com/j178/prek
```

**Enabling Hooks:**

```bash
# Run in the repository root
prek install
```

**Hook Execution Content:**

- `cargo fmt` - Code formatting
- `cargo check` - Compilation check
- `cargo clippy` - Lint check
- `markdownlint` - Markdown linting
- TOML/YAML validation

**Manual Execution:**

```bash
# Run on all files
prek run --all-files

# Run specific hooks only
prek run fmt
prek run clippy
```

**Compatibility with traditional pre-commit:**

`prek` is fully compatible with traditional `pre-commit`, so you can use your existing `.pre-commit-config.yaml` as is.

### Development Commands

```bash
# Run all checks (fmt, lint, test)
just check

# Run tests
cargo test

# Release build
cargo build --release
```

## 📚 Documentation

### For Users

- [README.md](README.md) - Project overview and basic functions
- [INSTALL.md](docs/INSTALL.md) - Installation methods
- [QUICKSTART.md](docs/QUICKSTART.md) - 5-minute guide
- [EXAMPLES.md](docs/EXAMPLES.md) - Collection of examples

<h3>For Developers</h3>

<ul>
<li><a href="CLAUDE.md">CLAUDE.md</a> - Project structure and architecture</li>
<li><a href="docs/testing/TESTING.md">TESTING.md</a> - Testing strategy and execution guide</li>
<li><a href="docs/testing/TEST_INVENTORY.md">TEST_INVENTORY.md</a> - Test inventory (list of all tests)</li>
<li><a href="docs/testing/COVERAGE.md">COVERAGE.md</a> - Coverage measurement guide</li>
<li><a href="docs/testing/GH_TESTING_EVALUATION.md">GH_TESTING_EVALUATION.md</a> - GitHub CLI related test evaluation</li>
<li><a href="docs/tests/">docs/tests/</a> - Functional verification test design documents (E2E tests)</li>
</ul>

<h2>📄 License</h2>

MIT License
