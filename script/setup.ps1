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
    Write-ColorOutput "=== 前提条件の確認 ===" "Cyan"

    # Check Rust
    if (!(Get-Command cargo -ErrorAction SilentlyContinue)) {
        Write-ColorOutput "エラー: Rustがインストールされていません" "Red"
        Write-ColorOutput "https://rustup.rs/ からインストールしてください" "Yellow"
        return $false
    }
    Write-ColorOutput "✓ Rust: $(cargo --version)" "Green"

    # Check GitHub CLI
    if (!(Get-Command gh -ErrorAction SilentlyContinue)) {
        Write-ColorOutput "エラー: GitHub CLI (gh) がインストールされていません" "Red"
        Write-ColorOutput "https://cli.github.com/ からインストールしてください" "Yellow"
        return $false
    }
    Write-ColorOutput "✓ GitHub CLI: $(gh --version | Select-Object -First 1)" "Green"

    return $true
}

# Install function
function Install-GistCache {
    Write-ColorOutput "`n=== gist-cache-rs のインストール ===" "Cyan"

    # Build release binary
    Write-ColorOutput "`nリリースビルドを実行しています..." "Cyan"
    cargo build --release

    if ($LASTEXITCODE -ne 0) {
        Write-ColorOutput "エラー: ビルドに失敗しました" "Red"
        exit 1
    }

    # Install
    Write-ColorOutput "`nインストールしています..." "Cyan"
    cargo install --path .

    if ($LASTEXITCODE -ne 0) {
        Write-ColorOutput "エラー: インストールに失敗しました" "Red"
        exit 1
    }

    Write-ColorOutput "`n✓ インストールが完了しました" "Green"
    Write-ColorOutput "  実行ファイル: $CargoHome\$BinaryName.exe" "White"

    # Check and configure PATH
    Write-ColorOutput "`n=== PATH設定の確認 ===" "Cyan"

    if (Test-PathInEnvironment $CargoHome) {
        Write-ColorOutput "✓ PATH設定済み: $CargoHome" "Green"
    } else {
        Write-ColorOutput "! CargoのbinディレクトリがPATHに含まれていません" "Yellow"
        $response = Read-Host "PATHに追加しますか？ (Y/n)"

        if ($response -eq "" -or $response -eq "Y" -or $response -eq "y") {
            if (Add-ToUserPath $CargoHome) {
                Write-ColorOutput "✓ PATHに追加しました: $CargoHome" "Green"
                Write-ColorOutput "  注意: 新しいターミナルセッションで有効になります" "Yellow"
            } else {
                Write-ColorOutput "✓ すでにPATHに含まれています" "Green"
            }
        } else {
            Write-ColorOutput "! 手動でPATHに追加してください: $CargoHome" "Yellow"
        }
    }

    # Initial cache update (optional)
    Write-ColorOutput "`n=== 初回キャッシュ作成 ===" "Cyan"
    $response = Read-Host "初回キャッシュを作成しますか？ (Y/n)"

    if ($response -eq "" -or $response -eq "Y" -or $response -eq "y") {
        Write-ColorOutput "`nキャッシュを作成しています..." "Cyan"
        & "$CargoHome\$BinaryName.exe" update --verbose

        if ($LASTEXITCODE -eq 0) {
            Write-ColorOutput "`n✓ キャッシュ作成が完了しました" "Green"
        } else {
            Write-ColorOutput "`n警告: キャッシュ作成に失敗しました" "Yellow"
            Write-ColorOutput "後で 'gist-cache-rs update' を実行してください" "Yellow"
        }
    }

    # Display usage
    Write-ColorOutput "`n=== 使用方法 ===" "Cyan"
    Write-ColorOutput "キャッシュ更新:" "White"
    Write-ColorOutput "  $BinaryName update" "Gray"
    Write-ColorOutput "`nGist実行:" "White"
    Write-ColorOutput "  $BinaryName run <query>" "Gray"
    Write-ColorOutput "`nヘルプ:" "White"
    Write-ColorOutput "  $BinaryName --help" "Gray"
}

# Uninstall function
function Uninstall-GistCache {
    Write-ColorOutput "`n=== gist-cache-rs のアンインストール ===" "Cyan"

    # Confirm
    $response = Read-Host "本当にアンインストールしますか？ (y/N)"
    if ($response -ne "y" -and $response -ne "Y") {
        Write-ColorOutput "キャンセルしました" "Yellow"
        return
    }

    # Check if binary exists
    if (!(Test-Path "$CargoHome\$BinaryName.exe")) {
        Write-ColorOutput "警告: バイナリが見つかりません" "Yellow"
        Write-ColorOutput "  パス: $CargoHome\$BinaryName.exe" "Gray"
    }

    # Uninstall binary
    Write-ColorOutput "`nバイナリをアンインストールしています..." "Cyan"
    cargo uninstall $BinaryName 2>&1 | Out-Null

    if ($LASTEXITCODE -eq 0) {
        Write-ColorOutput "✓ バイナリをアンインストールしました" "Green"
    } else {
        Write-ColorOutput "警告: アンインストールに失敗しました" "Yellow"
    }

    # Ask to remove cache
    $response = Read-Host "キャッシュも削除しますか？ ($CacheDir) (y/N)"
    if ($response -eq "y" -or $response -eq "Y") {
        if (Test-Path $CacheDir) {
            Remove-Item -Recurse -Force $CacheDir
            Write-ColorOutput "✓ キャッシュを削除しました" "Green"
        } else {
            Write-ColorOutput "キャッシュディレクトリが見つかりません" "Yellow"
        }
    }

    Write-ColorOutput "`n✓ アンインストールが完了しました" "Green"

    # PATH info
    Write-ColorOutput "`n=== PATH設定について ===" "Cyan"
    if (Test-PathInEnvironment $CargoHome) {
        Write-ColorOutput "注意: Cargo binディレクトリ ($CargoHome) はPATHに残っています" "Yellow"
        Write-ColorOutput "  他のCargoパッケージを使用している場合は削除しないでください" "Gray"
        Write-ColorOutput "  削除する場合は、システム環境変数から手動で削除してください" "Gray"
    }
}

# Main
Write-ColorOutput "gist-cache-rs セットアップスクリプト (Windows)" "Cyan"

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

