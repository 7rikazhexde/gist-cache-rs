# Installation Guide

## 📋 Prerequisites

### Required

- **Rust toolchain** (1.85 or later)

  **Linux / macOS:**

  ```bash
  rustc --version  # Verify
  ```

  Installation method:

  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

  **Windows:**

  ```powershell
  rustc --version  # Verify
  ```

  Installation method: Download from [rustup.rs](https://rustup.rs/)

- **GitHub CLI** (`gh`) - Authenticated

  **Linux / macOS:**

  ```bash
  gh --version     # Verify
  gh auth status   # Check authentication status
  ```

  **Windows:**

  ```powershell
  gh --version     # Verify
  gh auth status   # Check authentication status
  ```

  Authentication method:

  ```bash
  gh auth login
  ```

  Installation: [GitHub CLI](https://cli.github.com/)

### Recommended

- Git (for repository cloning)

## 🔧 Installation Methods

### Method 1: Setup Script (Recommended)

Interactively performs all steps.

#### Linux / macOS

```bash
# Clone the repository
git clone https://github.com/7rikazhexde/gist-cache-rs.git
cd gist-cache-rs

# Run the setup script
./script/setup.sh install
```

**Actions performed:**

1. ✅ Prerequisite check
2. 📁 Project directory detection
3. 🔨 Release build
4. 📦 Select installation method
5. ⚙️ Perform installation
6. 🔄 Initial cache creation
7. ⌨️ Alias setup (optional)

#### Windows

```powershell
# Clone the repository
git clone https://github.com/7rikazhexde/gist-cache-rs.git
cd gist-cache-rs

# Run the setup script
.\script\setup.ps1 install
```

**Actions performed:**

1. ✅ Prerequisite check (Rust, GitHub CLI)
2. 🔨 Release build
3. 📦 Perform cargo install
4. 🔄 Initial cache creation (optional)

**Installation location:**

- Binary: `%USERPROFILE%\.cargo\bin\gist-cache-rs.exe`
- Cache: `%LOCALAPPDATA%\gist-cache\`

### Method 2: cargo install (All platforms)

```bash
car go build --release
car go install --path .
```

**Installation location:**

- Linux/macOS: `~/.cargo/bin/gist-cache-rs`
- Windows: `%USERPROFILE%\.cargo\bin\gist-cache-rs.exe`

**PATH setting:**

**Linux / macOS:**
Usually set automatically. If not set:

```bash
# Add to ~/.bashrc or ~/.zshrc
export PATH="$HOME/.cargo/bin:$PATH"
source ~/.bashrc
```

**Windows:**
Cargo's bin directory is automatically added to PATH. If not set:

1. Open "Edit the system environment variables"
2. Click "Environment Variables..." button
3. Edit "Path" under User variables
4. Add `%USERPROFILE%\.cargo\bin`

### Method 3: System directory

```bash
car go build --release
sudo cp target/release/gist-cache-rs /usr/local/bin/
```

**Installation location:** `/usr/local/bin/gist-cache-rs`
**Feature:** Shared by all users, requires sudo privileges

### Method 4: User directory

```bash
car go build --release
mkdir -p ~/bin
cp target/release/gist-cache-rs ~/bin/
```

**Installation location:** `~/bin/gist-cache-rs`

**PATH setting:**

```bash
# Add to ~/.bashrc or ~/.zshrc
export PATH="$HOME/bin:$PATH"
source ~/.bashrc
```

### Method 5: Symbolic link (for developers)

```bash
car go build --release

# Link to /usr/local/bin (requires sudo)
sudo ln -sf "$(pwd)/target/release/gist-cache-rs" /usr/local/bin/gist-cache-rs

# Or link to ~/bin
mkdir -p ~/bin
ln -sf "$(pwd)/target/release/gist-cache-rs" ~/bin/gist-cache-rs
```

**Feature:** Automatically reflected after build, convenient for development

## ⚙️ Post-Installation Setup

### 1. Initial Cache Creation

```bash
gist-cache-rs update
```

Detailed display:

```bash
gist-cache-rs update --verbose
```

### 2. Alias Setting (Optional)

#### Automatic setting (when using setup.sh)

Set interactively during installation:

```bash
Use recommended alias names (gcrsu, gcrsr)? [Y/n]: y
```

Or

```bash
Use recommended alias names (gcrsu, gcrsr)? [Y/n]: n
Alias name for gist-cache-rs update: gcu
Alias name for gist-cache-rs run: gcr
```

#### Manual setting

```bash
# Add to ~/.bashrc or ~/.zshrc
alias gcrsu='gist-cache-rs update'
alias gcrsr='gist-cache-rs run'

# Apply settings
source ~/.bashrc
```

## ✅ Installation Verification

```bash
# Check version
gist-cache-rs --version

# Display help
gist-cache-rs --help

# Check cache status
gist-cache-rs update --verbose
```

## 🔍 Troubleshooting

### command not found: gist-cache-rs

**Cause:** PATH is not set

**Solution (Linux/macOS):**

```bash
# Check installation location
which gist-cache-rs

# Check PATH
echo $PATH

# If in ~/.cargo/bin
export PATH="$HOME/.cargo/bin:$PATH"

# If in ~/bin
export PATH="$HOME/bin:$PATH"

# Apply settings
source ~/.bashrc
```

**Solution (Windows):**

```powershell
# Check installation location
where.exe gist-cache-rs

# Check PATH
$env:PATH

# Set environment variable (PowerShell)
$env:PATH += ";$env:USERPROFILE\.cargo\bin"

# For persistent setting
[System.Environment]::SetEnvironmentVariable("Path", $env:PATH, [System.EnvironmentVariableTarget]::User)
```

### Permission error

**Cause:** No execution permission

**Solution:**

```bash
# Grant execution permission
chmod +x ~/.cargo/bin/gist-cache-rs
# Or
chmod +x /usr/local/bin/gist-cache-rs
# Or
chmod +x ~/bin/gist-cache-rs
```

### Build error

**Cause:** Outdated Rust version, dependency issues

**Solution:**

```bash
# Update Rust
rustup update

# Update dependencies
car go update

# Clean build
car go clean
car go build --release
```

### GitHub CLI authentication error

**Error:** `GitHub CLI (gh) is not authenticated`

**Solution:**

```bash
gh auth login
```

### PowerShell execution policy error (Windows)

**Error:** `This system\'s script execution is disabled...`

**Solution:**

```powershell
# Allow script execution for current user
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser

# Or bypass for specific script only
powershell -ExecutionPolicy Bypass -File .\script\setup.ps1 install
```

### Cache not created

**Error:** `Cache file not found`

**Solution:**

```bash
# Initial cache creation
gist-cache-rs update

# Display detailed information
gist-cache-rs update --verbose
```

### Rate limit error

**Warning:** `Rate limit remaining is low`

**Solution:**

- Wait for a while and retry
- Avoid `--force` option
- Use differential update

## 🗑️ Uninstallation

### Automatic Uninstallation

#### Linux / macOS

```bash
./script/setup.sh uninstall
```

Interactively select:

- Binary deletion
- Cache directory deletion
- Alias deletion

#### Windows

```powershell
.\script\setup.ps1 uninstall
```

Interactively select:

- Binary deletion
- Cache directory deletion

### Manual Uninstallation

#### Linux / macOS

```bash
# If installed with cargo
car go uninstall gist-cache-rs

# If installed in system directory
sudo rm /usr/local/bin/gist-cache-rs

# If installed in user directory
rm ~/bin/gist-cache-rs

# Delete cache directory
rm -rf ~/.cache/gist-cache/

# Remove aliases (delete relevant lines from ~/.bashrc or ~/.zshrc)
# Example:
# alias gcrsu='gist-cache-rs update'
# alias gcrsr='gist-cache-rs run'
```

#### Windows

```powershell
# If installed with cargo
car go uninstall gist-cache-rs

# Delete cache directory
Remove-Item -Recurse -Force "$env:LOCALAPPDATA\gist-cache"
```

## ➡️ Next Steps

- [QUICKSTART.md](QUICKSTART.md) - Quick Start Guide
- [EXAMPLES.md](EXAMPLES.md) - Practical Examples
- [README.md](../README.md) - Project Overview
