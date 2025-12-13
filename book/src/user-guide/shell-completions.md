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

## Installation Instructions

### Bash

**Linux / macOS:**

```bash
# Generate and install completion script
gist-cache-rs completions bash > ~/.local/share/bash-completion/completions/gist-cache-rs

# If the directory doesn't exist, create it first
mkdir -p ~/.local/share/bash-completion/completions
gist-cache-rs completions bash > ~/.local/share/bash-completion/completions/gist-cache-rs
```

If completions don't load automatically, add this to your `~/.bashrc`:

```bash
source ~/.local/share/bash-completion/completions/gist-cache-rs
```

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

```bash
# Fish automatically loads completions from this directory
gist-cache-rs completions fish > ~/.config/fish/completions/gist-cache-rs.fish
```

No additional configuration needed. Fish will automatically load the completion script.

### PowerShell

**Windows:**

```powershell
# Create scripts directory if it doesn't exist
New-Item -ItemType Directory -Force -Path ~\Documents\PowerShell\Scripts

# Generate completion script
gist-cache-rs completions powershell > ~\Documents\PowerShell\Scripts\gist-cache-rs.ps1

# Add to PowerShell profile
Add-Content $PROFILE "`n. ~\Documents\PowerShell\Scripts\gist-cache-rs.ps1"
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

## Verifying Installation

After installation, restart your shell or reload the configuration file. Test the completion by typing:

```bash
gist-cache-rs <TAB>
```

You should see available subcommands:

- `update`
- `run`
- `cache`
- `completions`
- `help`

Try completing subcommand options:

```bash
gist-cache-rs run --<TAB>
```

This should show available flags like `--preview`, `--interactive`, `--download`, etc.

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
