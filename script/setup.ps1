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

    # Display usage
    Write-ColorOutput "`n=== Usage ===" "Cyan"
    Write-ColorOutput "Cache update:" "White"
    Write-ColorOutput "  $BinaryName update" "Gray"
    Write-ColorOutput "`nGist execution:" "White"
    Write-ColorOutput "  $BinaryName run <query>" "Gray"
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
