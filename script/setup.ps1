# setup.ps1 - Windows installation script for gist-cache-rs
param(
    [Parameter(Position=0)]
    [ValidateSet("install", "uninstall")]
    [string]$Command = "install"
)

$ErrorActionPreference = "Stop"

# Constants
$BinaryName = "gist-cache-rs"
$CacheDir = "$env:LOCALAPPDATA\gist-cache"
$CargoHome = "$env:USERPROFILE\.cargo\bin"

# Colors (PowerShell)
function Write-ColorOutput {
    param(
        [Parameter(Mandatory=$true)]
        [string]$Message,
        [string]$Color = "White"
    )
    Write-Host $Message -ForegroundColor $Color
}

# Check if path is in PATH environment variable
function Test-PathInEnvironment {
    param([string]$PathToCheck)

    $currentPath = $env:Path
    $paths = $currentPath -split ';' | ForEach-Object { $_.Trim() }

    return $paths -contains $PathToCheck
}

# Add path to user PATH environment variable
function Add-ToUserPath {
    param([string]$PathToAdd)

    $userPath = [System.Environment]::GetEnvironmentVariable("Path", [System.EnvironmentVariableTarget]::User)

    if ($userPath -notlike "*$PathToAdd*") {
        $newPath = if ($userPath) { "$userPath;$PathToAdd" } else { $PathToAdd }
        [System.Environment]::SetEnvironmentVariable("Path", $newPath, [System.EnvironmentVariableTarget]::User)

        # Update current session
        $env:Path = "$env:Path;$PathToAdd"

        return $true
    }
    return $false
}

# Check prerequisites
function Test-Prerequisites {
    Write-ColorOutput "=== Prerequisites Check ===" "Cyan"

    # Check Rust
    if (!(Get-Command cargo -ErrorAction SilentlyContinue)) {
        Write-ColorOutput "Error: Rust is not installed" "Red"
        Write-ColorOutput "Install from https://rustup.rs/" "Yellow"
        return $false
    }
    Write-ColorOutput "✓ Rust: $(cargo --version)" "Green"

    # Check GitHub CLI
    if (!(Get-Command gh -ErrorAction SilentlyContinue)) {
        Write-ColorOutput "Error: GitHub CLI (gh) is not installed" "Red"
        Write-ColorOutput "Install from https://cli.github.com/" "Yellow"
        return $false
    }
    Write-ColorOutput "✓ GitHub CLI: $(gh --version | Select-Object -First 1)" "Green"

    return $true
}

# Install function
function Install-GistCache {
    Write-ColorOutput "`n=== Installing gist-cache-rs ===" "Cyan"

    # Build release binary
    Write-ColorOutput "`nPerforming release build..." "Cyan"
    cargo build --release

    if ($LASTEXITCODE -ne 0) {
        Write-ColorOutput "Error: Build failed" "Red"
        exit 1
    }

    # Install
    Write-ColorOutput "`nInstalling..." "Cyan"
    cargo install --path .

    if ($LASTEXITCODE -ne 0) {
        Write-ColorOutput "Error: Installation failed" "Red"
        exit 1
    }

    Write-ColorOutput "`n✓ Installation complete" "Green"
    Write-ColorOutput "  Executable: $CargoHome\$BinaryName.exe" "White"

    # Check and configure PATH
    Write-ColorOutput "`n=== PATH Configuration Check ===" "Cyan"

    if (Test-PathInEnvironment $CargoHome) {
        Write-ColorOutput "✓ PATH configured: $CargoHome" "Green"
    } else {
        Write-ColorOutput "! Cargo's bin directory is not included in PATH" "Yellow"
        $response = Read-Host "Add to PATH? (Y/n)"

        if ($response -eq "" -or $response -eq "Y" -or $response -eq "y") {
            if (Add-ToUserPath $CargoHome) {
                Write-ColorOutput "✓ Added to PATH: $CargoHome" "Green"
                Write-ColorOutput "  Note: Will be effective in new terminal sessions" "Yellow"
            } else {
                Write-ColorOutput "✓ Already included in PATH" "Green"
            }
        } else {
            Write-ColorOutput "! Please add to PATH manually: $CargoHome" "Yellow"
        }
    }

    # Initial cache update (optional)
    Write-ColorOutput "`n=== Initial Cache Creation ===" "Cyan"
    $response = Read-Host "Create initial cache? (Y/n)"

    if ($response -eq "" -or $response -eq "Y" -or $response -eq "y") {
        Write-ColorOutput "`nCreating cache..." "Cyan"
        & "$CargoHome\$BinaryName.exe" update --verbose

        if ($LASTEXITCODE -eq 0) {
            Write-ColorOutput "`n✓ Cache creation complete" "Green"
        } else {
            Write-ColorOutput "`nWarning: Cache creation failed" "Yellow"
            Write-ColorOutput "Please run 'gist-cache-rs update' later" "Yellow"
        }
    }

    # Setup Shell Completion
    Write-ColorOutput "`n=== Shell Completion Setup ===" "Cyan"
    $response = Read-Host "Set up shell completion for PowerShell? (Y/n)"

    if ($response -eq "" -or $response -eq "Y" -or $response -eq "y") {
        Write-ColorOutput "`nConfiguring PowerShell completion..." "Cyan"

        # Define paths
        $CompletionScriptDir = Join-Path -Path $HOME -ChildPath "Documents\PowerShell\Scripts"
        $CompletionScriptPath = Join-Path -Path $CompletionScriptDir -ChildPath "gist-cache-rs.ps1"

        # Check if profile exists and if completion is already configured
        $isConfigured = $false
        if (Test-Path $PROFILE) {
            $profileContent = Get-Content $PROFILE -ErrorAction SilentlyContinue
            if ($profileContent -match "gist-cache-rs.ps1") {
                $isConfigured = $true
            }
        }

        # Logic based on whether it's configured or not
        if ($isConfigured) {
            Write-ColorOutput "✓ Completion script seems to be already configured. Updating..." "Green"

            # Just update the script file
            & "$CargoHome\$BinaryName.exe" completions powershell > $CompletionScriptPath

            Write-ColorOutput "✓ Completion script updated." "Green"
            Write-ColorOutput "  It will be active in new terminal sessions." "Yellow"

        } else {
            Write-ColorOutput "Setting up new completion configuration..." "White"

            # 1. Create directory if it doesn't exist
            if (-not (Test-Path -Path $CompletionScriptDir)) {
                New-Item -ItemType Directory -Force -Path $CompletionScriptDir
                Write-ColorOutput "  Created directory: $CompletionScriptDir" "Gray"
            }

            # 2. Generate completion script
            & "$CargoHome\$BinaryName.exe" completions powershell > $CompletionScriptPath
            Write-ColorOutput "  Generated script: $CompletionScriptPath" "Gray"

            # 3. Add to PowerShell profile
            if (-not (Test-Path $PROFILE)) {
                New-Item -Path $PROFILE -ItemType File -Force
                Write-ColorOutput "  Created profile: $PROFILE" "Gray"
            }
            Add-Content $PROFILE "`n. `"$CompletionScriptPath`""
            Write-ColorOutput "  Added script to profile: $PROFILE" "Gray"

            Write-ColorOutput "`n✓ Shell completion setup is complete." "Green"
            Write-ColorOutput "  Please restart your terminal or run '. `$PROFILE' to activate." "Yellow"
        }
    }

    # Configuration File Setup
    Write-ColorOutput "`n=== Configuration File Setup ===" "Cyan"

    $ConfigDir = "$env:APPDATA\gist-cache"
    $ConfigFile = "$ConfigDir\config.toml"

    if (Test-Path $ConfigFile) {
        Write-ColorOutput "✓ Configuration file already exists: $ConfigFile" "Green"

        $response = Read-Host "View current configuration? (Y/n)"
        if ($response -eq "" -or $response -eq "Y" -or $response -eq "y") {
            Write-Host ""
            & "$CargoHome\$BinaryName.exe" config show
            Write-Host ""
        }

        # Ask if user wants to update existing config
        $response = Read-Host "Update configuration? (y/N)"
        if ($response -eq "y" -or $response -eq "Y") {
            $shouldSetupConfig = $true
        } else {
            Write-ColorOutput "Configuration retained without changes" "Yellow"
            $shouldSetupConfig = $false
        }
    } else {
        $response = Read-Host "Set up default configuration? (Y/n)"
        $shouldSetupConfig = ($response -eq "" -or $response -eq "Y" -or $response -eq "y")
    }

    if ($shouldSetupConfig) {
        Write-ColorOutput "`nConfiguring default settings..." "Cyan"

        # Create config directory if it doesn't exist
        if (-not (Test-Path $ConfigDir)) {
            New-Item -ItemType Directory -Force -Path $ConfigDir | Out-Null
        }

        # Configure interpreters for each extension
        $configApplied = $false

        Write-ColorOutput "Configure interpreters for each file extension:" "White"
        Write-ColorOutput "(Press Enter to skip any extension)" "Cyan"
        Write-Host ""

        # Python (.py)
        Write-ColorOutput "Python files (.py):" "White"
        Write-ColorOutput "  1) uv (recommended for modern Python)" "White"
        Write-ColorOutput "  2) python3" "White"
        Write-ColorOutput "  3) Skip" "White"
        $pyChoice = Read-Host "Selection [1-3]"
        switch ($pyChoice) {
            "1" {
                & "$CargoHome\$BinaryName.exe" config set defaults.interpreter.py uv 2>&1 | Out-Null
                if ($LASTEXITCODE -eq 0) {
                    Write-ColorOutput "✓ Python (.py) → uv" "Green"
                    $configApplied = $true
                }
            }
            "2" {
                & "$CargoHome\$BinaryName.exe" config set defaults.interpreter.py python3 2>&1 | Out-Null
                if ($LASTEXITCODE -eq 0) {
                    Write-ColorOutput "✓ Python (.py) → python3" "Green"
                    $configApplied = $true
                }
            }
        }
        Write-Host ""

        # Ruby (.rb)
        $response = Read-Host "Configure Ruby interpreter for .rb files? (y/N)"
        if ($response -eq "y" -or $response -eq "Y") {
            & "$CargoHome\$BinaryName.exe" config set defaults.interpreter.rb ruby 2>&1 | Out-Null
            if ($LASTEXITCODE -eq 0) {
                Write-ColorOutput "✓ Ruby (.rb) → ruby" "Green"
                $configApplied = $true
            }
        }
        Write-Host ""

        # JavaScript (.js)
        $response = Read-Host "Configure Node.js interpreter for .js files? (y/N)"
        if ($response -eq "y" -or $response -eq "Y") {
            & "$CargoHome\$BinaryName.exe" config set defaults.interpreter.js node 2>&1 | Out-Null
            if ($LASTEXITCODE -eq 0) {
                Write-ColorOutput "✓ JavaScript (.js) → node" "Green"
                $configApplied = $true
            }
        }
        Write-Host ""

        # TypeScript (.ts)
        Write-ColorOutput "TypeScript files (.ts):" "White"
        Write-ColorOutput "  1) ts-node" "White"
        Write-ColorOutput "  2) deno" "White"
        Write-ColorOutput "  3) bun" "White"
        Write-ColorOutput "  4) Skip" "White"
        $tsChoice = Read-Host "Selection [1-4]"
        switch ($tsChoice) {
            "1" {
                & "$CargoHome\$BinaryName.exe" config set defaults.interpreter.ts ts-node 2>&1 | Out-Null
                if ($LASTEXITCODE -eq 0) {
                    Write-ColorOutput "✓ TypeScript (.ts) → ts-node" "Green"
                    $configApplied = $true
                }
            }
            "2" {
                & "$CargoHome\$BinaryName.exe" config set defaults.interpreter.ts deno 2>&1 | Out-Null
                if ($LASTEXITCODE -eq 0) {
                    Write-ColorOutput "✓ TypeScript (.ts) → deno" "Green"
                    $configApplied = $true
                }
            }
            "3" {
                & "$CargoHome\$BinaryName.exe" config set defaults.interpreter.ts bun 2>&1 | Out-Null
                if ($LASTEXITCODE -eq 0) {
                    Write-ColorOutput "✓ TypeScript (.ts) → bun" "Green"
                    $configApplied = $true
                }
            }
        }
        Write-Host ""

        # Shell scripts (.sh)
        Write-ColorOutput "Shell scripts (.sh):" "White"
        Write-ColorOutput "  1) bash (recommended)" "White"
        Write-ColorOutput "  2) sh" "White"
        Write-ColorOutput "  3) zsh" "White"
        Write-ColorOutput "  4) Skip" "White"
        $shChoice = Read-Host "Selection [1-4]"
        switch ($shChoice) {
            "1" {
                & "$CargoHome\$BinaryName.exe" config set defaults.interpreter.sh bash 2>&1 | Out-Null
                if ($LASTEXITCODE -eq 0) {
                    Write-ColorOutput "✓ Shell (.sh) → bash" "Green"
                    $configApplied = $true
                }
            }
            "2" {
                & "$CargoHome\$BinaryName.exe" config set defaults.interpreter.sh sh 2>&1 | Out-Null
                if ($LASTEXITCODE -eq 0) {
                    Write-ColorOutput "✓ Shell (.sh) → sh" "Green"
                    $configApplied = $true
                }
            }
            "3" {
                & "$CargoHome\$BinaryName.exe" config set defaults.interpreter.sh zsh 2>&1 | Out-Null
                if ($LASTEXITCODE -eq 0) {
                    Write-ColorOutput "✓ Shell (.sh) → zsh" "Green"
                    $configApplied = $true
                }
            }
        }
        Write-Host ""

        # PHP (.php)
        $response = Read-Host "Configure PHP interpreter for .php files? (y/N)"
        if ($response -eq "y" -or $response -eq "Y") {
            & "$CargoHome\$BinaryName.exe" config set defaults.interpreter.php php 2>&1 | Out-Null
            if ($LASTEXITCODE -eq 0) {
                Write-ColorOutput "✓ PHP (.php) → php" "Green"
                $configApplied = $true
            }
        }
        Write-Host ""

        # Perl (.pl)
        $response = Read-Host "Configure Perl interpreter for .pl files? (y/N)"
        if ($response -eq "y" -or $response -eq "Y") {
            & "$CargoHome\$BinaryName.exe" config set defaults.interpreter.pl perl 2>&1 | Out-Null
            if ($LASTEXITCODE -eq 0) {
                Write-ColorOutput "✓ Perl (.pl) → perl" "Green"
                $configApplied = $true
            }
        }
        Write-Host ""

        # PowerShell (.ps1)
        $response = Read-Host "Configure PowerShell interpreter for .ps1 files? (y/N)"
        if ($response -eq "y" -or $response -eq "Y") {
            & "$CargoHome\$BinaryName.exe" config set defaults.interpreter.ps1 pwsh 2>&1 | Out-Null
            if ($LASTEXITCODE -eq 0) {
                Write-ColorOutput "✓ PowerShell (.ps1) → pwsh" "Green"
                $configApplied = $true
            }
        }
        Write-Host ""

        # Wildcard fallback (*)
        Write-ColorOutput "Fallback interpreter (for unrecognized extensions):" "White"
        Write-ColorOutput "  1) bash (recommended)" "White"
        Write-ColorOutput "  2) python3" "White"
        Write-ColorOutput "  3) pwsh" "White"
        Write-ColorOutput "  4) Skip" "White"
        $fallbackChoice = Read-Host "Selection [1-4]"
        switch ($fallbackChoice) {
            "1" {
                & "$CargoHome\$BinaryName.exe" config set "defaults.interpreter.*" bash 2>&1 | Out-Null
                if ($LASTEXITCODE -eq 0) {
                    Write-ColorOutput "✓ Fallback (*) → bash" "Green"
                    $configApplied = $true
                }
            }
            "2" {
                & "$CargoHome\$BinaryName.exe" config set "defaults.interpreter.*" python3 2>&1 | Out-Null
                if ($LASTEXITCODE -eq 0) {
                    Write-ColorOutput "✓ Fallback (*) → python3" "Green"
                    $configApplied = $true
                }
            }
            "3" {
                & "$CargoHome\$BinaryName.exe" config set "defaults.interpreter.*" pwsh 2>&1 | Out-Null
                if ($LASTEXITCODE -eq 0) {
                    Write-ColorOutput "✓ Fallback (*) → pwsh" "Green"
                    $configApplied = $true
                }
            }
        }
        Write-Host ""

        # Additional configuration options
        $response = Read-Host "Configure execution confirmation (safety feature)? (y/N)"
        if ($response -eq "y" -or $response -eq "Y") {
            & "$CargoHome\$BinaryName.exe" config set execution.confirm_before_run true 2>&1 | Out-Null
            if ($LASTEXITCODE -eq 0) {
                Write-ColorOutput "✓ Execution confirmation enabled" "Green"
                $configApplied = $true
            }
        }

        if (-not $configApplied) {
            Write-ColorOutput "No configuration was applied" "Yellow"
        }

        Write-ColorOutput "`n✓ Configuration setup complete" "Green"

        if (Test-Path $ConfigFile) {
            Write-ColorOutput "  Configuration file: $ConfigFile" "Gray"

            $response = Read-Host "View configuration? (Y/n)"
            if ($response -eq "" -or $response -eq "Y" -or $response -eq "y") {
                Write-Host ""
                & "$CargoHome\$BinaryName.exe" config show
            }
        }

        Write-ColorOutput "`nTo modify configuration later, use:" "White"
        Write-ColorOutput "  $BinaryName config set <key> <value>" "Gray"
        Write-ColorOutput "  $BinaryName config show" "Gray"
    }

    # Display usage
    Write-ColorOutput "`n=== Usage ===" "Cyan"
    Write-ColorOutput "Cache update:" "White"
    Write-ColorOutput "  $BinaryName update" "Gray"
    Write-ColorOutput "`nGist execution:" "White"
    Write-ColorOutput "  $BinaryName run <query>" "Gray"
    Write-ColorOutput "`nConfiguration:" "White"
    Write-ColorOutput "  $BinaryName config show" "Gray"
    Write-ColorOutput "`nHelp:" "White"
    Write-ColorOutput "  $BinaryName --help" "Gray"
}

# Uninstall function
function Uninstall-GistCache {
    Write-ColorOutput "`n=== Uninstalling gist-cache-rs ===" "Cyan"

    # Confirm
    $response = Read-Host "Are you sure you want to uninstall? (y/N)"
    if ($response -ne "y" -and $response -ne "Y") {
        Write-ColorOutput "Cancelled" "Yellow"
        return
    }

    # Check if binary exists
    if (!(Test-Path "$CargoHome\$BinaryName.exe")) {
        Write-ColorOutput "Warning: Binary not found" "Yellow"
        Write-ColorOutput "  Path: $CargoHome\$BinaryName.exe" "Gray"
    }

    # Uninstall binary
    Write-ColorOutput "`nUninstalling binary..." "Cyan"
    cargo uninstall $BinaryName 2>&1 | Out-Null

    if ($LASTEXITCODE -eq 0) {
        Write-ColorOutput "✓ Binary uninstalled" "Green"
    } else {
        Write-ColorOutput "Warning: Uninstallation failed" "Yellow"
    }

    # Ask to remove cache
    $response = Read-Host "Delete cache as well? ($CacheDir) (y/N)"
    if ($response -eq "y" -or $response -eq "Y") {
        if (Test-Path $CacheDir) {
            Remove-Item -Recurse -Force $CacheDir
            Write-ColorOutput "✓ Cache deleted" "Green"
        } else {
            Write-ColorOutput "Cache directory not found" "Yellow"
        }
    }

    # Ask to remove config
    $ConfigDir = "$env:APPDATA\gist-cache"
    $response = Read-Host "Delete configuration as well? ($ConfigDir) (y/N)"
    if ($response -eq "y" -or $response -eq "Y") {
        if (Test-Path $ConfigDir) {
            Remove-Item -Recurse -Force $ConfigDir
            Write-ColorOutput "✓ Configuration deleted" "Green"
        } else {
            Write-ColorOutput "Configuration directory not found" "Yellow"
        }
    }

    Write-ColorOutput "`n✓ Uninstallation complete" "Green"

    # PATH info
    Write-ColorOutput "`n=== About PATH Configuration ===" "Cyan"
    if (Test-PathInEnvironment $CargoHome) {
        Write-ColorOutput "Note: Cargo bin directory ($CargoHome) remains in PATH" "Yellow"
        Write-ColorOutput "  Do not delete if you are using other Cargo packages" "Gray"
        Write-ColorOutput "  If you want to delete it, please remove it manually from system environment variables" "Gray"
    }
}

# Main
Write-ColorOutput "gist-cache-rs Setup Script (Windows)" "Cyan"

# Check prerequisites
if (!(Test-Prerequisites)) {
    exit 1
}

# Execute command
switch ($Command) {
    "install" {
        Install-GistCache
    }
    "uninstall" {
        Uninstall-GistCache
    }
}
