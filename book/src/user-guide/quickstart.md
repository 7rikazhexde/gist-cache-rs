# Quick Start Guide

A minimal guide to get started with `gist-cache-rs` in 5 minutes.

## Step 1: Verify Prerequisites

```bash
# Check if Rust is installed
rustc --version

# Check if GitHub CLI is installed
gh --version

# Check if authenticated with GitHub CLI
gh auth status
```

If not installed, please refer to the [Installation Guide](installation.md).

## Step 2: Installation

```bash
# Clone the repository
git clone https://github.com/7rikazhexde/gist-cache-rs.git
cd gist-cache-rs

# Build and install
cargo build --release
cargo install --path .

# Verify installation
gist-cache-rs --version
```

For other installation methods, please refer to the [Installation Guide](installation.md).

## Step 3: Initial Cache Creation

```bash
# Create cache (with progress display)
gist-cache-rs update

# With detailed output (recommended)
gist-cache-rs update --verbose
```

**Example Output (Normal Mode with Progress Display):**

```bash
Updating Gist cache...
⠙ Fetching Gist information from GitHub API...
Fetched 42 Gists
[████████████████████████████████] 42/42 (100%)
Cache update completed
Total Gists: 42
```

**Example Output (Verbose Mode):**

```bash
Updating Gist cache...
Mode: Force full update
Rate limit remaining: 4999
Fetching Gist information from GitHub API...
Gists fetched: 42
New/Updated: 42 items
Cache update completed
Total Gists: 42
```

**Note:** The normal mode displays a spinner while fetching Gists and a progress bar when processing multiple Gists (10+). Use `--verbose` for detailed logs instead of progress indicators.

## Step 4: Search and Execute Gist

### Preview (Check content without executing)

```bash
# Search by keyword and preview
gist-cache-rs run --preview backup
```

### Actually Execute

```bash
# Execute a Bash script
gist-cache-rs run backup bash

# Execute a Python script
gist-cache-rs run data_analysis.py python3

# Execute a Python script with uv
gist-cache-rs run ml-script uv
```

### Execute with Arguments

```bash
# Pass arguments to a script
gist-cache-rs run backup bash /src /dst

# Pass arguments to a Python script
gist-cache-rs run data_analysis.py python3 input.csv --output result.json
```

## Step 5: Configuration (Optional)

Set up your preferences with the config command:

### Method 1: Interactive Setup (Recommended - v0.8.7+)

```bash
# Interactive configuration with cursor-based UI
gist-cache-rs config setting

# Follow the prompts to:
# - Select interpreters for each file extension
# - Configure execution settings
# - Set cache retention period
```

### Method 2: Manual Commands

```bash
# Set default interpreter (saves time)
gist-cache-rs config set defaults.interpreter python3

# Enable execution confirmation for safety
gist-cache-rs config set execution.confirm_before_run true

# View your configuration
gist-cache-rs config show
```

For more configuration options, see the [Configuration Guide](configuration.md).

## Step 6: Alias Setting (Optional)

To use it more conveniently, set up aliases:

```bash
# Add to ~/.bashrc
echo 'alias gcrsu="gist-cache-rs update"' >> ~/.bashrc
echo 'alias gcrsr="gist-cache-rs run"' >> ~/.bashrc
source ~/.bashrc

# Now you can use the shortened forms
gcrsu                # Update cache
gcrsr backup bash    # Execute Gist
```

## Frequently Used Commands

### Configuration Management

```bash
# Interactive configuration setup (recommended)
gist-cache-rs config setting

# Set configuration values
gist-cache-rs config set defaults.interpreter python3
gist-cache-rs config set execution.confirm_before_run true
gist-cache-rs config set cache.retention_days 30

# Get configuration values
gist-cache-rs config get defaults.interpreter

# Show all configuration
gist-cache-rs config show

# Edit config file
gist-cache-rs config edit

# Reset to defaults
gist-cache-rs config reset
```

### Cache Management

```bash
# Differential update (normal)
gist-cache-rs update

# Force full update
gist-cache-rs update --force

# Detailed display
gist-cache-rs update --verbose
```

### Content Cache Management

```bash
# Display cache list
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

### Gist Search and Execution

```bash
# Basic search and execution
gist-cache-rs run keyword

# Preview (check content without executing)
gist-cache-rs run -p keyword

# Interactive mode (when using read command, etc.)
gist-cache-rs run -i interactive-script

# Save to download folder
gist-cache-rs run --download backup bash

# Download after preview
gist-cache-rs run -p --download script

# Search by filename
gist-cache-rs run --filename setup.sh

# Search by description
gist-cache-rs run --description deployment

# Get latest information before execution (force update)
gist-cache-rs run --force backup bash
```

### Specify Interpreter

Argument specifications depend on the script.

```bash
# Bash (default)
gist-cache-rs run script bash arg1 arg2 ...

# Python3
gist-cache-rs run script python3 arg1 arg2 ...

# Ruby
gist-cache-rs run script ruby arg1 arg2 ...

# Node.js
gist-cache-rs run script node arg1 arg2 ...

# uv (PEP 723 compatible)
gist-cache-rs run script uv arg1 arg2 ...
```

## Practical Examples

Please check the [Usage Examples](examples.md).

## Troubleshooting

### Cache not found

```bash
# Error: Cache file not found
→ Run gist-cache-rs update
```

### GitHub authentication error

```bash
# Error: GitHub CLI is not authenticated
→ Run gh auth login
```

### Command not found

```bash
# If command not found
→ Check path with which gist-cache-rs
→ Check if ~/.cargo/bin or /usr/local/bin is in PATH
```

### Search results not found

```bash
# Cache might be old
→ Update with gist-cache-rs update
```

## Related Information

- [Installation Guide](installation.md) - Installation details and troubleshooting
- [Configuration Guide](configuration.md) - Customize your settings
- [Usage Examples](examples.md) - Practical examples and actual usage

## Help

```bash
# Overall help
gist-cache-rs --help

# Subcommand help
gist-cache-rs update --help
gist-cache-rs run --help

# Running without arguments also displays help
gist-cache-rs run
```
