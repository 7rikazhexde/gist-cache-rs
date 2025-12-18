# Shell Completions

`gist-cache-rs` supports shell completions for Bash, Zsh, Fish, and PowerShell. This enables auto-completion of commands, subcommands, and options by pressing the Tab key, significantly improving the command-line experience.

## Supported Shells

- **Bash** - Linux, macOS, Windows (Git Bash, WSL)
- **Zsh** - macOS (default), Linux
- **Fish** - Linux, macOS
- **PowerShell** - Windows, Linux, macOS (PowerShell Core)

## Generating Completion Scripts

Use the `completions` subcommand to generate shell-specific completion scripts:

```bash
gist-cache-rs completions <SHELL>
```

Where `<SHELL>` is one of: `bash`, `zsh`, `fish`, or `powershell`.

## Before Installation

### Backup Your Configuration

Before modifying shell configuration files, it's recommended to create backups:

**Bash:**

```bash
# Backup .bashrc
cp ~/.bashrc ~/.bashrc.backup
```

**Zsh:**

```bash
# Backup .zshrc
cp ~/.zshrc ~/.zshrc.backup
```

**Fish:**

```bash
# Backup Fish config
cp -r ~/.config/fish ~/.config/fish.backup
```

**PowerShell:**

```powershell
# Backup PowerShell profile (if exists)
if (Test-Path $PROFILE) {
    Copy-Item $PROFILE "$PROFILE.backup"
}
```

### Restore from Backup

If something goes wrong, you can restore your configuration:

**Bash/Zsh:**

```bash
# Restore .bashrc or .zshrc
cp ~/.bashrc.backup ~/.bashrc
# or
cp ~/.zshrc.backup ~/.zshrc

# Reload configuration
source ~/.bashrc  # or source ~/.zshrc
```

**Fish:**

```bash
# Restore Fish config
rm -rf ~/.config/fish
cp -r ~/.config/fish.backup ~/.config/fish
```

**PowerShell:**

```powershell
# Restore PowerShell profile
Copy-Item "$PROFILE.backup" $PROFILE

# Reload profile
. $PROFILE
```

## Installation Instructions

### Bash

**Linux / macOS:**

1. **Generate Completion Script:**
    Create the directory for completions and generate the script.

    ```bash
    # Create the directory if it doesn't exist
    mkdir -p ~/.local/share/bash-completion/completions

    # Generate and install the completion script
    gist-cache-rs completions bash > ~/.local/share/bash-completion/completions/gist-cache-rs
    ```

2. **Activate Completions:**
    The `bash-completion` package should automatically load the script. If completions do not work after **starting a new shell session**, you may need to source it manually in your `~/.bashrc`.

    ```bash
    # Add this line to ~/.bashrc if completions are not loading
    echo 'source ~/.local/share/bash-completion/completions/gist-cache-rs' >> ~/.bashrc
    ```

    To apply the change immediately, run `source ~/.bashrc`.

**Windows (Git Bash):**

```bash
# Create completions directory
mkdir -p ~/bash-completion/completions

# Generate completion script
gist-cache-rs completions bash > ~/bash-completion/completions/gist-cache-rs

# Add to ~/.bashrc
echo 'source ~/bash-completion/completions/gist-cache-rs' >> ~/.bashrc
```

### Zsh

**macOS / Linux:**

```bash
# Create completion directory if it doesn't exist
mkdir -p ~/.zfunc

# Generate completion script
gist-cache-rs completions zsh > ~/.zfunc/_gist-cache-rs
```

Add to your `~/.zshrc` (if not already present):

```bash
# Add custom completion directory to fpath
fpath=(~/.zfunc $fpath)

# Initialize completion system
autoload -Uz compinit && compinit
```

Reload your shell or run:

```bash
source ~/.zshrc
```

### Fish

**Linux / macOS:**

Fish automatically loads completion scripts from its completions directory.

```bash
# Generate and place the completion script
gist-cache-rs completions fish > ~/.config/fish/completions/gist-cache-rs.fish
```

The changes will take effect when you start a new Fish session. No additional configuration is needed.

### PowerShell

**Windows:**

1. **Generate Completion Script:**
    First, ensure the target directory exists and then generate the script.

    ```powershell
    # Create a directory for PowerShell scripts if it doesn't exist
    $scriptDir = ~\Documents\PowerShell\Scripts
    if (-not (Test-Path $scriptDir)) {
        New-Item -ItemType Directory -Force -Path $scriptDir
    }

    # Generate the completion script
    gist-cache-rs completions powershell > $scriptDir\gist-cache-rs.ps1
    ```

2. **Update PowerShell Profile:**
    Next, add the script to your PowerShell profile to have it load automatically.

    ```powershell
    # Create the profile if it doesn't exist
    if (-not (Test-Path $PROFILE)) {
        New-Item -Path $PROFILE -ItemType File -Force
    }

    # Add the script loading command to your profile
    Add-Content $PROFILE "`n. $scriptDir\gist-cache-rs.ps1"
    ```

3. **Activate Completions:**
    The changes will take effect when you start a new PowerShell session. To apply them immediately in your current session, run:

    ```powershell
    . $PROFILE
    ```

    **Note on Execution Policy:** If you encounter an error, your script execution policy might be too restrictive. You can check it with `Get-ExecutionPolicy`. If it's `Restricted`, you may need to change it. For example:

    ```powershell
    # This allows locally created scripts to run
    Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
    ```

**Linux / macOS (PowerShell Core):**

```powershell
# Create scripts directory
mkdir -p ~/.config/powershell/scripts

# Generate completion script
gist-cache-rs completions powershell > ~/.config/powershell/scripts/gist-cache-rs.ps1

# Add to profile
Add-Content $PROFILE "`n. ~/.config/powershell/scripts/gist-cache-rs.ps1"
```

## Configuration Files

After installation, the following files will be created:

### Bash

**Completion script location:**

```bash
~/.local/share/bash-completion/completions/gist-cache-rs
```

**Profile configuration (~/.bashrc):**

```bash
# gist-cache-rs shell completion (if manually added)
source ~/.local/share/bash-completion/completions/gist-cache-rs
```

### Zsh

**Completion script location:**

```bash
~/.zfunc/_gist-cache-rs
```

**Profile configuration (~/.zshrc):**

```bash
# Add custom completion directory to fpath
fpath=(~/.zfunc $fpath)

# Initialize completion system
autoload -Uz compinit && compinit
```

### Fish

**Completion script location:**

```bash
~/.config/fish/completions/gist-cache-rs.fish
```

**No profile configuration needed** - Fish automatically loads completions from this directory.

### PowerShell

**Completion script location (Windows):**

```powershell
~\Documents\PowerShell\Scripts\gist-cache-rs.ps1
```

**Profile configuration ($PROFILE):**

```powershell
# gist-cache-rs shell completion
. ~\Documents\PowerShell\Scripts\gist-cache-rs.ps1
```

**PowerShell profile location:**

- Windows: `~\Documents\PowerShell\Microsoft.PowerShell_profile.ps1`
- Linux/macOS: `~/.config/powershell/Microsoft.PowerShell_profile.ps1`

## Verifying Installation

After installation, restart your shell or reload the configuration file to test the completion.

### Bash/Zsh (WSL2/Linux/macOS)

**Complete subcommands:**

```bash
$ gist-cache-rs [Tab]
-h           -V           --help       --version
update       run          cache        config       completions  help
```

**Complete options:**

```bash
$ gist-cache-rs run --[Tab]
--interactive  --preview      --force        --download
--id           --filename     --description  --help
```

**Auto-complete partial input:**

```bash
gist-cache-rs ru[Tab]
gist-cache-rs run    # ← Auto-completed
```

**View help after completion:**

```bash
$ gist-cache-rs run --help
Search from cache and execute

Usage: gist-cache-rs run [OPTIONS] [QUERY] [INTERPRETER] [SCRIPT_ARGS]...

Arguments:
  [QUERY]           Search keyword (ID, filename, or description)
  [INTERPRETER]     Interpreter or execution command (bash, python3, uv, etc.)
  [SCRIPT_ARGS]...  Additional arguments to pass to the script

Options:
  -i, --interactive  Interactive script execution mode
  -p, --preview      Preview mode (display content only)
  -f, --force        Update Gist cache before execution
      --download     Save file to download folder
      --id           Direct ID specification mode
      --filename     Search by filename
      --description  Search by description
  -h, --help         Print help
```

### PowerShell (Windows)

**Complete subcommands** (press Tab after typing):

```powershell
PS> gist-cache-rs [Tab]
# Cycles through: update, run, cache, config, completions, help, -h, --help, -V, --version
```

**Complete options:**

```powershell
PS> gist-cache-rs run --[Tab]
# Cycles through: --interactive, --preview, --force, --download, --id, --filename, --description, --help
```

**Auto-complete partial input:**

```powershell
PS> gist-cache-rs ru[Tab]
PS> gist-cache-rs run    # ← Auto-completed
```

**Complete cache subcommands:**

```powershell
PS> gist-cache-rs cache [Tab]
# Cycles through: list, size, clean, clear, help
```

## Troubleshooting

### Completions Not Working (Bash)

1. Verify the completion script exists:

   ```bash
   ls -l ~/.local/share/bash-completion/completions/gist-cache-rs
   ```

2. Check if bash-completion is installed:

   ```bash
   # Ubuntu/Debian
   sudo apt install bash-completion

   # macOS (via Homebrew)
   brew install bash-completion@2
   ```

3. Reload your shell:

   ```bash
   source ~/.bashrc
   ```

### Completions Not Working (Zsh)

1. Verify `compinit` is called in `~/.zshrc`:

   ```bash
   grep compinit ~/.zshrc
   ```

2. Check the fpath includes your completion directory:

   ```bash
   echo $fpath
   ```

3. Rebuild completion cache:

   ```bash
   rm ~/.zcompdump
   compinit
   ```

### Completions Not Working (Fish)

1. Verify the completion script exists:

   ```bash
   ls -l ~/.config/fish/completions/gist-cache-rs.fish
   ```

2. Reload Fish completions:

   ```bash
   fish_update_completions
   ```

### Completions Not Working (PowerShell)

1. Check execution policy:

   ```powershell
   Get-ExecutionPolicy
   ```

   If it's `Restricted`, change it:

   ```powershell
   Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
   ```

2. Verify profile exists and is loaded:

   ```powershell
   Test-Path $PROFILE
   cat $PROFILE
   ```

3. Reload profile:

   ```powershell
   . $PROFILE
   ```

## Updating Completions

When you update `gist-cache-rs` to a new version with new commands or options, regenerate the completion scripts using the same installation commands shown above. The new script will overwrite the old one.

## Editing Shell Configuration

If you need to modify or review your shell configuration:

### Edit Configuration Files

**Bash:**

```bash
# Edit .bashrc with your preferred editor
nano ~/.bashrc
# or
vim ~/.bashrc
# or
code ~/.bashrc  # VS Code
```

**Zsh:**

```bash
# Edit .zshrc
nano ~/.zshrc
# or
vim ~/.zshrc
```

**Fish:**

```bash
# Edit Fish config
nano ~/.config/fish/config.fish
# or use Fish's web-based configuration
fish_config
```

**PowerShell:**

```powershell
# Edit PowerShell profile
notepad $PROFILE
# or
code $PROFILE  # VS Code
```

### Verify Configuration Content

Check what was added to your configuration files:

**Bash:**

```bash
# View .bashrc content related to gist-cache-rs
grep -A 2 "gist-cache-rs" ~/.bashrc
```

**Zsh:**

```bash
# View .zshrc content related to completion setup
grep -A 3 "gist-cache-rs\|compinit" ~/.zshrc
```

**PowerShell:**

```powershell
# View profile content related to gist-cache-rs
Get-Content $PROFILE | Select-String -Pattern "gist-cache-rs" -Context 0,2
```

### Apply Changes

After editing, reload the configuration:

**Bash:**

```bash
source ~/.bashrc
```

**Zsh:**

```bash
source ~/.zshrc
```

**Fish:**

```bash
source ~/.config/fish/config.fish
```

**PowerShell:**

```powershell
. $PROFILE
```

## Uninstalling Completions

To remove shell completions:

**Bash:**

```bash
rm ~/.local/share/bash-completion/completions/gist-cache-rs
```

**Zsh:**

```bash
rm ~/.zfunc/_gist-cache-rs
```

**Fish:**

```bash
rm ~/.config/fish/completions/gist-cache-rs.fish
```

**PowerShell:**

```powershell
# Windows
Remove-Item ~\Documents\PowerShell\Scripts\gist-cache-rs.ps1

# Linux/macOS
rm ~/.config/powershell/scripts/gist-cache-rs.ps1
```

Don't forget to remove the corresponding lines from your shell's configuration file (`.bashrc`, `.zshrc`, or PowerShell profile).
