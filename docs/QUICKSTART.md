# ⚡ Quick Start Guide

A minimal guide to get started with `gist-cache-rs` in 5 minutes.

## Step 1: ✅ Verify Prerequisites

```bash
# Check if Rust is installed
rustc --version

# Check if GitHub CLI is installed
gh --version

# Check if authenticated with GitHub CLI
gh auth status
```

If not installed, please refer to [INSTALL.md](INSTALL.md).

## Step 2: 📥 Installation

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

For other installation methods, please refer to [INSTALL.md](INSTALL.md).

## Step 3: 🔄 Initial Cache Creation

```bash
# Create cache
gist-cache-rs update

# With detailed output (recommended)
gist-cache-rs update --verbose
```

**Example Output:**

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

## Step 4: 🚀 Search and Execute Gist

### 👁️ Preview (Check content without executing)

```bash
# Search by keyword and preview
gist-cache-rs run --preview backup
```

### ▶️ Actually Execute

```bash
# Execute a Bash script
gist-cache-rs run backup bash

# Execute a Python script
gist-cache-rs run data_analysis.py python3

# Execute a Python script with uv
gist-cache-rs run ml-script uv
```

### 📝 Execute with Arguments

```bash
# Pass arguments to a script
gist-cache-rs run backup bash /src /dst

# Pass arguments to a Python script
gist-cache-rs run data_analysis.py python3 input.csv --output result.json
```

## Step 5: ⚡ Alias Setting (Optional)

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

## 📚 Frequently Used Commands

### 🔄 Cache Management

```bash
# Differential update (normal)
gist-cache-rs update

# Force full update
gist-cache-rs update --force

# Detailed display
gist-cache-rs update --verbose
```

### 🗂️ Content Cache Management

```bash
# Display cache list
gist-cache-rs cache list

# Check cache size
gist-cache-rs cache size

# Clear all caches
gist-cache-rs cache clear
```

### 🔍 Gist Search and Execution

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

### 🔧 Specify Interpreter

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

## 💼 Practical Examples

Please check [EXAMPLES.md](EXAMPLES.md).

## 🔧 Troubleshooting

### ❌ Cache not found

```bash
# Error: Cache file not found
→ Run gist-cache-rs update
```

### 🔐 GitHub authentication error

```bash
# Error: GitHub CLI is not authenticated
→ Run gh auth login
```

### 🚫 Command not found

```bash
# If command not found
→ Check path with which gist-cache-rs
→ Check if ~/.cargo/bin or /usr/local/bin is in PATH
```

### 🔎 Search results not found

```bash
# Cache might be old
→ Update with gist-cache-rs update
```

## 🎯 Related Information

- [README.md](../README.md) - Detailed feature description
- [INSTALL.md](INSTALL.md) - Installation details
- [EXAMPLES.md](EXAMPLES.md) - Practical examples (actual usage examples)

## ❓ Help

```bash
# Overall help
gist-cache-rs --help

# Subcommand help
gist-cache-rs update --help
gist-cache-rs run --help

# Running without arguments also displays help
gist-cache-rs run
```
