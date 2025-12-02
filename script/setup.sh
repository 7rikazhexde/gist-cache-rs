#!/usr/bin/env bash

set -e

# Color definitions
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
BOLD='\033[1m'
NC='\033[0m' # No Color

# Constants
REPO_URL="https://github.com/7rikazhexde/gist-cache-rs.git"
BINARY_NAME="gist-cache-rs"
CACHE_DIR="$HOME/.cache/gist-cache"

# Script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Mode detection
MODE="${1:-install}"

# Check if running interactively (stdin is a terminal)
IS_INTERACTIVE=false
if [ -t 0 ]; then
    IS_INTERACTIVE=true
fi

# Environment variables for non-interactive mode
# GIST_CACHE_INSTALL_METHOD: 1-5 (default: 1 = cargo install)
# GIST_CACHE_SKIP_CACHE: true/false (default: false)
# GIST_CACHE_SKIP_ALIAS: true/false (default: false)
# GIST_CACHE_AUTO_ALIAS: true/false (default: false) - 非対話モードでエイリアス自動設定
# GIST_CACHE_ALIAS_UPDATE: alias name for update (default: gcrsu)
# GIST_CACHE_ALIAS_RUN: alias name for run (default: gcrsr)
INSTALL_METHOD="${GIST_CACHE_INSTALL_METHOD:-1}"
SKIP_CACHE_UPDATE="${GIST_CACHE_SKIP_CACHE:-false}"
SKIP_ALIAS="${GIST_CACHE_SKIP_ALIAS:-false}"
AUTO_ALIAS="${GIST_CACHE_AUTO_ALIAS:-false}"
ALIAS_UPDATE="${GIST_CACHE_ALIAS_UPDATE:-gcrsu}"
ALIAS_RUN="${GIST_CACHE_ALIAS_RUN:-gcrsr}"

# Functions
print_usage() {
    cat << EOF
使用方法: $0 [COMMAND]

COMMAND:
  install     インストール（デフォルト）
  uninstall   アンインストール
  help        このヘルプを表示

インストール例:
  # ローカルで実行
  ./setup.sh install

  # curlで直接実行（非対話モード）
  curl -sSL https://raw.githubusercontent.com/7rikazhexde/gist-cache-rs/main/script/setup.sh | bash

  # curlで実行（環境変数でカスタマイズ）
  curl -sSL https://raw.githubusercontent.com/7rikazhexde/gist-cache-rs/main/script/setup.sh | GIST_CACHE_INSTALL_METHOD=1 bash

  # アンインストール
  curl -sSL https://raw.githubusercontent.com/7rikazhexde/gist-cache-rs/main/script/setup.sh | bash -s uninstall

環境変数:
  GIST_CACHE_INSTALL_METHOD  インストール方法 (1-5, デフォルト: 1)
    1: cargo install (推奨)
    2: システムディレクトリ (/usr/local/bin)
    3: ユーザーディレクトリ (~/bin)
    4: シンボリックリンク
    5: スキップ
  GIST_CACHE_SKIP_CACHE      キャッシュ更新をスキップ (true/false, デフォルト: false)
  GIST_CACHE_SKIP_ALIAS      エイリアス設定をスキップ (true/false, デフォルト: false)
  GIST_CACHE_AUTO_ALIAS      非対話モードでエイリアス自動設定 (true/false, デフォルト: false)
  GIST_CACHE_ALIAS_UPDATE    updateコマンドのエイリアス名 (デフォルト: gcrsu)
  GIST_CACHE_ALIAS_RUN       runコマンドのエイリアス名 (デフォルト: gcrsr)

EOF
}

print_header() {
    echo -e "\n${CYAN}${BOLD}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${CYAN}${BOLD}  $1${NC}"
    echo -e "${CYAN}${BOLD}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}\n"
}

print_success() {
    echo -e "${GREEN}✓${NC} $1"
}

print_error() {
    echo -e "${RED}✗${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}⚠${NC} $1"
}

print_info() {
    echo -e "${BLUE}ℹ${NC} $1"
}

# Enhanced confirm function with non-interactive support
confirm() {
    local prompt="$1"
    local default="${2:-n}"

    # Non-interactive mode: use default
    if [ "$IS_INTERACTIVE" = false ]; then
        if [ "$default" = "y" ]; then
            return 0
        else
            return 1
        fi
    fi

    # Interactive mode: ask user
    local response
    if [ "$default" = "y" ]; then
        prompt="$prompt [Y/n]: "
    else
        prompt="$prompt [y/N]: "
    fi

    read -r -p "$(echo -e "${YELLOW}${prompt}${NC}")" response
    response="${response:-$default}"

    case "$response" in
        [yY][eE][sS]|[yY]) return 0 ;;
        *) return 1 ;;
    esac
}

check_command() {
    if command -v "$1" &> /dev/null; then
        print_success "$1 がインストールされています ($(command -v "$1"))"
        return 0
    else
        print_error "$1 が見つかりません"
        return 1
    fi
}

check_version() {
    local cmd="$1"
    local version_flag="${2:---version}"

    echo -e "  ${BLUE}バージョン:${NC} $($cmd "$version_flag" 2>&1 | head -n 1)"
}

# Uninstall function
uninstall_gist_cache_rs() {
    clear
    echo -e "${RED}${BOLD}"
    cat << "EOF"
   ____ _     _        ____           _
  / ___(_)___| |_     / ___|__ _  ___| |__   ___
 | |  _| / __| __|   | |   / _` |/ __| '_ \ / _ \
 | |_| | \__ \ |_    | |__| (_| | (__| | | |  __/
  \____|_|___/\__|____\____\__,_|\___|_| |_|\___|
              |_____|
              Uninstall Script
EOF
    echo -e "${NC}"

    print_warning "このスクリプトは gist-cache-rs をアンインストールします"
    echo ""

    # 非対話モードでは確認をスキップ
    if [ "$IS_INTERACTIVE" = true ]; then
        if ! confirm "アンインストールを開始しますか？" "n"; then
            echo "アンインストールを中止しました"
            exit 0
        fi
    else
        print_info "非対話モード: アンインストールを開始します"
    fi

    print_header "アンインストール処理"

    UNINSTALLED=false

    # Check and remove from cargo install
    if [ -f "$HOME/.cargo/bin/$BINARY_NAME" ]; then
        print_info "cargo でインストールされたバイナリを削除中..."
        if cargo uninstall "$BINARY_NAME" 2>/dev/null; then
            print_success "cargo uninstall が完了しました"
            UNINSTALLED=true
        else
            print_warning "cargo uninstall に失敗しました（手動削除を試みます）"
            if rm -f "$HOME/.cargo/bin/$BINARY_NAME"; then
                print_success "$HOME/.cargo/bin/$BINARY_NAME を削除しました"
                UNINSTALLED=true
            fi
        fi
    fi

    # Check and remove from /usr/local/bin
    if [ -f "/usr/local/bin/$BINARY_NAME" ]; then
        print_info "/usr/local/bin からバイナリを削除中..."
        if sudo rm -f "/usr/local/bin/$BINARY_NAME"; then
            print_success "/usr/local/bin/$BINARY_NAME を削除しました"
            UNINSTALLED=true
        else
            print_error "削除に失敗しました（権限が必要です）"
        fi
    fi

    # Check and remove from ~/bin
    if [ -f "$HOME/bin/$BINARY_NAME" ]; then
        print_info "$HOME/bin からバイナリを削除中..."
        if rm -f "$HOME/bin/$BINARY_NAME"; then
            print_success "$HOME/bin/$BINARY_NAME を削除しました"
            UNINSTALLED=true
        fi
    fi

    if [ "$UNINSTALLED" = false ]; then
        print_warning "$BINARY_NAME のインストールが見つかりませんでした"
    fi

    # Ask about cache directory
    echo ""
    if [ -d "$CACHE_DIR" ]; then
        print_info "キャッシュディレクトリを検出: $CACHE_DIR"

        SHOULD_DELETE_CACHE=false
        if [ "$IS_INTERACTIVE" = false ]; then
            # 非対話モードでは削除
            print_info "非対話モード: キャッシュディレクトリを削除します"
            SHOULD_DELETE_CACHE=true
        elif confirm "キャッシュディレクトリを削除しますか？" "n"; then
            SHOULD_DELETE_CACHE=true
        fi

        if [ "$SHOULD_DELETE_CACHE" = true ]; then
            if rm -rf "$CACHE_DIR"; then
                print_success "キャッシュディレクトリを削除しました"
            else
                print_error "キャッシュディレクトリの削除に失敗しました"
            fi
        else
            print_info "キャッシュディレクトリは保持されます"
        fi
    fi

    # Ask about aliases
    echo ""
    print_info "エイリアス設定の確認"

    for rcfile in "$HOME/.bashrc" "$HOME/.zshrc" "$HOME/.config/fish/config.fish"; do
        if [ ! -f "$rcfile" ]; then
            continue
        fi

        # gist-cache-rs 関連のエイリアスを検出（コマンド内容で検索）
        FOUND_ALIASES=()

        # 'gist-cache-rs update' を含むエイリアスを検索
        while IFS= read -r line; do
            if [[ "$line" =~ ^[[:space:]]*alias[[:space:]]+([^=]+)=[[:space:]]*[\'\"]*gist-cache-rs[[:space:]]+update ]]; then
                ALIAS_NAME="${BASH_REMATCH[1]}"
                FOUND_ALIASES+=("${ALIAS_NAME}:update")
            fi
        done < "$rcfile"

        # 'gist-cache-rs run' を含むエイリアスを検索
        while IFS= read -r line; do
            if [[ "$line" =~ ^[[:space:]]*alias[[:space:]]+([^=]+)=[[:space:]]*[\'\"]*gist-cache-rs[[:space:]]+run ]]; then
                ALIAS_NAME="${BASH_REMATCH[1]}"
                FOUND_ALIASES+=("${ALIAS_NAME}:run")
            fi
        done < "$rcfile"

        # エイリアスが見つかった場合
        if [ ${#FOUND_ALIASES[@]} -gt 0 ]; then
            print_warning "$rcfile にエイリアス設定が残っています"

            # 検出されたエイリアスを表示
            echo "  検出されたエイリアス:"
            for alias_entry in "${FOUND_ALIASES[@]}"; do
                ALIAS_NAME="${alias_entry%%:*}"
                ALIAS_TYPE="${alias_entry##*:}"
                if [ "$ALIAS_TYPE" = "update" ]; then
                    echo "    - ${ALIAS_NAME} → gist-cache-rs update"
                else
                    echo "    - ${ALIAS_NAME} → gist-cache-rs run"
                fi
            done
            echo ""

            SHOULD_DELETE_ALIAS=false
            if [ "$IS_INTERACTIVE" = false ]; then
                # 非対話モードでは削除
                print_info "非対話モード: エイリアスを削除します"
                SHOULD_DELETE_ALIAS=true
            elif confirm "$rcfile からエイリアスを削除しますか？" "n"; then
                SHOULD_DELETE_ALIAS=true
            fi

            if [ "$SHOULD_DELETE_ALIAS" = true ]; then
                # Create backup
                BACKUP_FILE="${rcfile}.backup.$(date +%Y%m%d%H%M%S)"
                cp "$rcfile" "$BACKUP_FILE"

                # マーカーコメントを削除（複数パターンに対応）
                sed -i.tmp '/# gist-cache-rs aliases/d' "$rcfile"

                # 検出された各エイリアスを削除
                for alias_entry in "${FOUND_ALIASES[@]}"; do
                    ALIAS_NAME="${alias_entry%%:*}"
                    # エイリアス名をエスケープ（特殊文字に対応）
                    ESCAPED_ALIAS=$(printf '%s\n' "$ALIAS_NAME" | sed 's/[]\/$*.^[]/\\&/g')
                    sed -i.tmp "/^[[:space:]]*alias[[:space:]]\+${ESCAPED_ALIAS}=/d" "$rcfile"
                done

                # 一時ファイルを削除
                rm -f "${rcfile}.tmp"

                print_success "エイリアスを削除しました（バックアップ: $BACKUP_FILE）"
            fi
        fi
    done

    print_header "アンインストール完了"
    print_success "gist-cache-rs のアンインストールが完了しました"
    echo ""
    print_info "ご利用ありがとうございました！"
    echo ""
}


# ============================================================================
# Mode selection
# ============================================================================
case "$MODE" in
    help|--help|-h)
        print_usage
        exit 0
        ;;
    uninstall)
        uninstall_gist_cache_rs
        exit 0
        ;;
    install)
        # Continue to installation
        ;;
    *)
        print_error "無効なコマンド: $MODE"
        echo ""
        print_usage
        exit 1
        ;;
esac

# ============================================================================
# Installation process
# ============================================================================

clear
echo -e "${CYAN}${BOLD}"
cat << "EOF"
   ____ _     _        ____           _
  / ___(_)___| |_     / ___|__ _  ___| |__   ___
 | |  _| / __| __|   | |   / _` |/ __| '_ \ / _ \
 | |_| | \__ \ |_    | |__| (_| | (__| | | |  __/
  \____|_|___/\__|____\____\__,_|\___|_| |_|\___|
              |_____|
                  Setup Script
EOF
echo -e "${NC}"

print_info "このスクリプトは gist-cache-rs のセットアップを行います"

# Non-interactive mode notification
if [ "$IS_INTERACTIVE" = false ]; then
    echo ""
    print_warning "非対話モードで実行中（デフォルト設定を使用）"
    print_info "カスタマイズする場合は環境変数を設定してください"
    print_info "詳細: $0 help"
    echo ""
else
    echo ""
    if ! confirm "セットアップを開始しますか？" "y"; then
        echo "セットアップを中止しました"
        exit 0
    fi
fi

# ============================================================================
# Step 1: 前提条件の確認
# ============================================================================
print_header "Step 1: 前提条件の確認"

PREREQUISITES_OK=true

# Check Rust
echo -e "${BOLD}Rust Toolchain:${NC}"
if check_command "rustc" && check_command "cargo"; then
    check_version "rustc" "--version"
    check_version "cargo" "--version"

    # Check minimum version (1.85)
    RUST_VERSION=$(rustc --version | grep -oP '\d+\.\d+' | head -1)
    if [ "$(echo "$RUST_VERSION >= 1.85" | bc -l 2>/dev/null || echo 0)" -eq 1 ] 2>/dev/null; then
        print_success "Rustのバージョンは要件を満たしています (>= 1.85)"
    else
        print_warning "Rustのバージョンが古い可能性があります（推奨: 1.85以降）"
    fi
else
    print_error "Rustがインストールされていません"
    print_info "インストール方法: https://rustup.rs/"
    print_info "コマンド: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    PREREQUISITES_OK=false
fi

echo ""

# Check GitHub CLI
echo -e "${BOLD}GitHub CLI:${NC}"
if check_command "gh"; then
    check_version "gh" "--version"

    # Check authentication
    if gh auth status &> /dev/null; then
        print_success "GitHub CLI は認証済みです"
        GH_USER=$(gh api user --jq .login 2>/dev/null || echo "unknown")
        echo -e "  ${BLUE}ユーザー:${NC} $GH_USER"
    else
        print_error "GitHub CLI が認証されていません"
        print_info "認証コマンド: gh auth login"
        PREREQUISITES_OK=false
    fi
else
    print_error "GitHub CLI (gh) がインストールされていません"
    print_info "インストール方法: https://cli.github.com/"
    PREREQUISITES_OK=false
fi

echo ""

if [ "$PREREQUISITES_OK" = false ]; then
    print_error "前提条件が満たされていません"
    echo ""
    if [ "$IS_INTERACTIVE" = true ]; then
        if confirm "それでも続行しますか？" "n"; then
            print_warning "前提条件が不足した状態で続行します"
        else
            echo "セットアップを中止しました"
            exit 1
        fi
    else
        print_error "非対話モードでは前提条件が必須です"
        exit 1
    fi
fi

print_success "前提条件チェック完了"

# ============================================================================
# Step 2: プロジェクトディレクトリの確認
# ============================================================================
print_header "Step 2: プロジェクトディレクトリの確認"

# Cargo.tomlが存在するか確認
if [ -f "$SCRIPT_DIR/../Cargo.toml" ]; then
    print_success "プロジェクトディレクトリを検出: $(dirname "$SCRIPT_DIR")"
    PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
elif [ -f "$SCRIPT_DIR/Cargo.toml" ]; then
    print_success "プロジェクトディレクトリを検出: $SCRIPT_DIR"
    PROJECT_DIR="$SCRIPT_DIR"
elif [ -f "./Cargo.toml" ]; then
    print_success "プロジェクトディレクトリを検出: $(pwd)"
    PROJECT_DIR="$(pwd)"
else
    print_warning "Cargo.toml が見つかりません"
    echo ""

    # Non-interactive mode: automatically clone
    if [ "$IS_INTERACTIVE" = false ]; then
        print_info "非対話モード: 自動的にリポジトリをクローンします"
        SHOULD_CLONE=true
    else
        if confirm "GitHubからプロジェクトをクローンしますか？" "y"; then
            SHOULD_CLONE=true
        else
            SHOULD_CLONE=false
        fi
    fi

    if [ "$SHOULD_CLONE" = true ]; then
        # Check if git is available
        if ! command -v git &> /dev/null; then
            print_error "git コマンドが見つかりません"
            print_info "git をインストールしてから再実行してください"
            exit 1
        fi

        # Create temp directory
        TEMP_DIR=$(mktemp -d)
        print_info "一時ディレクトリ: $TEMP_DIR"

        # Clone repository
        echo ""
        print_info "リポジトリをクローン中..."
        if git clone "$REPO_URL" "$TEMP_DIR/gist-cache-rs"; then
            print_success "クローンが完了しました"
            PROJECT_DIR="$TEMP_DIR/gist-cache-rs"
            CLEANUP_TEMP=true
        else
            print_error "クローンに失敗しました"
            rm -rf "$TEMP_DIR"
            exit 1
        fi
    else
        echo ""
        read -r -p "$(echo -e "${YELLOW}プロジェクトディレクトリのパスを入力してください: ${NC}")" PROJECT_DIR

        if [ ! -f "$PROJECT_DIR/Cargo.toml" ]; then
            print_error "指定されたディレクトリに Cargo.toml が見つかりません"
            exit 1
        fi
    fi
fi

cd "$PROJECT_DIR" || exit 1
print_info "作業ディレクトリ: $(pwd)"


# ============================================================================
# Step 3: ビルド
# ============================================================================
print_header "Step 3: ビルド"

if [ "$IS_INTERACTIVE" = false ]; then
    print_info "非対話モード: 自動的にリリースビルドを実行します"
    SHOULD_BUILD=true
else
    if confirm "リリースビルドを実行しますか？" "y"; then
        SHOULD_BUILD=true
    else
        SHOULD_BUILD=false
    fi
fi

if [ "$SHOULD_BUILD" = true ]; then
    echo ""
    print_info "ビルドを開始します (時間がかかる場合があります)..."
    echo ""

    if cargo build --release; then
        print_success "ビルドが完了しました"

        # バイナリのサイズを表示
        if [ -f "target/release/gist-cache-rs" ]; then
            BINARY_SIZE=$(du -h target/release/gist-cache-rs | cut -f1)
            print_info "バイナリサイズ: $BINARY_SIZE"
        fi
    else
        print_error "ビルドに失敗しました"
        exit 1
    fi
else
    print_warning "ビルドをスキップしました"

    if [ ! -f "target/release/gist-cache-rs" ]; then
        print_error "ビルド済みバイナリが見つかりません"
        exit 1
    fi
fi

# ============================================================================
# Step 4: インストール
# ============================================================================
print_header "Step 4: インストール"

if [ "$IS_INTERACTIVE" = false ]; then
    print_info "非対話モード: インストール方法 ${INSTALL_METHOD} を使用"
    INSTALL_CHOICE="$INSTALL_METHOD"
else
    echo "インストール方法を選択してください:"
    echo "  1) cargo install (推奨) - ~/.cargo/bin にインストール"
    echo "  2) システムディレクトリ - /usr/local/bin にコピー (要sudo)"
    echo "  3) ユーザーディレクトリ - ~/bin にコピー"
    echo "  4) シンボリックリンク - 開発者向け"
    echo "  5) スキップ"
    echo ""

    read -r -p "$(echo -e "${YELLOW}選択 [1-5]: ${NC}")" INSTALL_CHOICE

    # Validate input
    if [[ ! "$INSTALL_CHOICE" =~ ^[1-5]$ ]]; then
        print_error "無効な選択です"
        exit 1
    fi
fi

case $INSTALL_CHOICE in
    1)
        print_info "cargo install を実行します..."
        if cargo install --path .; then
            print_success "インストールが完了しました"
            print_info "インストール先: ~/.cargo/bin/gist-cache-rs"

            # PATH確認
            if echo "$PATH" | grep -q ".cargo/bin"; then
                print_success "$HOME/.cargo/bin はPATHに含まれています"
            else
                print_warning "$HOME/.cargo/bin がPATHに含まれていません"
                print_info "以下を ~/.bashrc または ~/.zshrc に追加してください:"
                echo -e "  ${CYAN}export PATH=\"\$HOME/.cargo/bin:\$PATH\"${NC}"
            fi
        else
            print_error "インストールに失敗しました"
            exit 1
        fi
        ;;
    2)
        print_info "システムディレクトリにコピーします..."
        if sudo cp target/release/gist-cache-rs /usr/local/bin/; then
            print_success "インストールが完了しました"
            print_info "インストール先: /usr/local/bin/gist-cache-rs"
        else
            print_error "インストールに失敗しました"
            exit 1
        fi
        ;;
    3)
        print_info "ユーザーディレクトリにコピーします..."
        mkdir -p ~/bin
        if cp target/release/gist-cache-rs ~/bin/; then
            print_success "インストールが完了しました"
            print_info "インストール先: ~/bin/gist-cache-rs"

            # PATH確認
            if echo "$PATH" | grep -q "$HOME/bin"; then
                print_success "$HOME/bin はPATHに含まれています"
            else
                print_warning "$HOME/bin がPATHに含まれていません"
                print_info "以下を ~/.bashrc または ~/.zshrc に追加してください:"
                echo -e "  ${CYAN}export PATH=\"\$HOME/bin:\$PATH\"${NC}"
            fi
        else
            print_error "インストールに失敗しました"
            exit 1
        fi
        ;;
    4)
        print_info "シンボリックリンクを作成します..."

        if [ "$IS_INTERACTIVE" = false ]; then
            # Non-interactive: default to ~/bin
            LINK_CHOICE=2
        else
            echo "  1) /usr/local/bin (要sudo)"
            echo "  2) ~/bin"
            read -r -p "$(echo -e "${YELLOW}選択 [1-2]: ${NC}")" LINK_CHOICE
        fi

        case $LINK_CHOICE in
            1)
                if sudo ln -sf "$(pwd)/target/release/gist-cache-rs" /usr/local/bin/gist-cache-rs; then
                    print_success "シンボリックリンクを作成しました"
                    print_info "リンク先: /usr/local/bin/gist-cache-rs"
                fi
                ;;
            2)
                mkdir -p ~/bin
                if ln -sf "$(pwd)/target/release/gist-cache-rs" ~/bin/gist-cache-rs; then
                    print_success "シンボリックリンクを作成しました"
                    print_info "リンク先: ~/bin/gist-cache-rs"
                fi
                ;;
        esac
        ;;
    5)
        print_warning "インストールをスキップしました"
        ;;
    *)
        print_error "無効な選択です"
        exit 1
        ;;
esac

# ============================================================================
# Step 5: インストール確認
# ============================================================================
print_header "Step 5: インストール確認"

if command -v gist-cache-rs &> /dev/null; then
    print_success "gist-cache-rs コマンドが利用可能です"
    check_version "gist-cache-rs" "--version"
    print_info "パス: $(which gist-cache-rs)"
else
    print_warning "gist-cache-rs コマンドが見つかりません"
    print_info "シェルを再起動するか、PATHを更新してください"
fi

# ============================================================================
# Step 6: 初回キャッシュ作成
# ============================================================================
print_header "Step 6: 初回キャッシュ作成"

if [ "$SKIP_CACHE_UPDATE" = "true" ]; then
    print_info "環境変数により、キャッシュ更新をスキップします"
elif ! command -v gist-cache-rs &> /dev/null; then
    print_warning "コマンドが利用できないため、キャッシュ作成をスキップします"
    print_info "後で 'gist-cache-rs update' を実行してください"
else
    if [ "$IS_INTERACTIVE" = false ]; then
        print_info "非対話モード: 自動的にキャッシュ更新を実行します"
        SHOULD_UPDATE=true
    else
        if confirm "初回キャッシュ更新を実行しますか？" "y"; then
            SHOULD_UPDATE=true
        else
            SHOULD_UPDATE=false
        fi
    fi

    if [ "$SHOULD_UPDATE" = true ]; then
        echo ""
        print_info "キャッシュ更新を開始します..."
        echo ""

        if gist-cache-rs update --verbose; then
            print_success "キャッシュ更新が完了しました"
        else
            print_error "キャッシュ更新に失敗しました"
        fi
    else
        print_info "後で 'gist-cache-rs update' を実行してください"
    fi
fi

# ============================================================================
# Step 7: エイリアス設定（オプション）
# ============================================================================
print_header "Step 7: エイリアス設定（オプション）"

if [ "$SKIP_ALIAS" = "true" ]; then
    print_info "環境変数により、エイリアス設定をスキップします"
else
    if [ "$IS_INTERACTIVE" = false ]; then
        # 非対話モード
        if [ "$AUTO_ALIAS" = "true" ]; then
            # 自動設定を実行
            print_info "非対話モード: エイリアス自動設定を実行します"
            echo ""

            # シェルの検出（環境変数SHELLから判定）
            if [[ "$SHELL" == *"zsh"* ]]; then
                SHELL_RC="$HOME/.zshrc"
            elif [[ "$SHELL" == *"bash"* ]]; then
                SHELL_RC="$HOME/.bashrc"
            else
                SHELL_RC="$HOME/.bashrc"
            fi

            print_info "設定ファイル: $SHELL_RC"
            print_info "設定するエイリアス:"
            echo -e "  ${CYAN}alias ${ALIAS_UPDATE}='gist-cache-rs update'${NC}"
            echo -e "  ${CYAN}alias ${ALIAS_RUN}='gist-cache-rs run'${NC}"
            echo ""

            # 既存のエイリアスをチェック
            UPDATE_EXISTS=false
            RUN_EXISTS=false

            if [ -f "$SHELL_RC" ]; then
                if grep -q "^[[:space:]]*alias[[:space:]]\+${ALIAS_UPDATE}=" "$SHELL_RC"; then
                    UPDATE_EXISTS=true
                fi
                if grep -q "^[[:space:]]*alias[[:space:]]\+${ALIAS_RUN}=" "$SHELL_RC"; then
                    RUN_EXISTS=true
                fi
            fi

            # エイリアスを追加（既存のものはスキップ）
            ADDED=false
            SKIPPED=false

            # マーカーコメントを追加（新規追加がある場合のみ）
            if [ "$UPDATE_EXISTS" = false ] || [ "$RUN_EXISTS" = false ]; then
                echo "" >> "$SHELL_RC"
                echo "# gist-cache-rs aliases: ${ALIAS_UPDATE}, ${ALIAS_RUN} (added on $(date +%Y-%m-%d))" >> "$SHELL_RC"
            fi

            # update エイリアスを追加
            if [ "$UPDATE_EXISTS" = true ]; then
                print_warning "エイリアス '${ALIAS_UPDATE}' は既に存在します（スキップ）"
                SKIPPED=true
            else
                echo "alias ${ALIAS_UPDATE}='gist-cache-rs update'" >> "$SHELL_RC"
                ADDED=true
            fi

            # run エイリアスを追加
            if [ "$RUN_EXISTS" = true ]; then
                print_warning "エイリアス '${ALIAS_RUN}' は既に存在します（スキップ）"
                SKIPPED=true
            else
                echo "alias ${ALIAS_RUN}='gist-cache-rs run'" >> "$SHELL_RC"
                ADDED=true
            fi

            # 結果を表示
            if [ "$ADDED" = true ]; then
                print_success "エイリアスを追加しました"
                print_info "反映するには以下を実行してください:"
                echo -e "  ${CYAN}source $SHELL_RC${NC}"
            fi
            if [ "$SKIPPED" = true ]; then
                print_info "既存のエイリアスは保持されました"
            fi
        else
            # 自動設定しない場合は手動設定を案内
            print_info "非対話モード: エイリアス設定をスキップします"
            print_info "手動で設定する場合:"
            echo -e "  ${CYAN}alias gcrsu='gist-cache-rs update'${NC}"
            echo -e "  ${CYAN}alias gcrsr='gist-cache-rs run'${NC}"
            echo ""
            print_info "または、環境変数で自動設定:"
            echo -e "  ${CYAN}GIST_CACHE_AUTO_ALIAS=true${NC}"
        fi
    elif confirm "便利なエイリアスを設定しますか？" "y"; then
        echo ""
        echo "推奨エイリアス:"
        echo -e "  ${CYAN}alias gcrsu='gist-cache-rs update'${NC}"
        echo -e "  ${CYAN}alias gcrsr='gist-cache-rs run'${NC}"
        echo ""

        # シェルの検出
        if [ -n "$BASH_VERSION" ]; then
            SHELL_RC="$HOME/.bashrc"
        elif [ -n "$ZSH_VERSION" ]; then
            SHELL_RC="$HOME/.zshrc"
        else
            SHELL_RC="$HOME/.bashrc"
        fi

        print_info "設定ファイル: $SHELL_RC"
        echo ""

        # 推奨エイリアスを使うか確認
        if confirm "推奨エイリアス名（gcrsu, gcrsr）を使用しますか？" "y"; then
            ALIAS_UPDATE="gcrsu"
            ALIAS_RUN="gcrsr"
        else
            # カスタムエイリアス名を入力
            echo ""
            print_info "カスタムエイリアス名を入力してください"
            echo ""

            read -r -p "$(echo -e "${YELLOW}gist-cache-rs update 用のエイリアス名: ${NC}")" ALIAS_UPDATE
            if [ -z "$ALIAS_UPDATE" ]; then
                ALIAS_UPDATE="gcrsu"
                print_warning "入力がないため、デフォルト名 'gcrsu' を使用します"
            fi

            read -r -p "$(echo -e "${YELLOW}gist-cache-rs run 用のエイリアス名: ${NC}")" ALIAS_RUN
            if [ -z "$ALIAS_RUN" ]; then
                ALIAS_RUN="gcrsr"
                print_warning "入力がないため、デフォルト名 'gcrsr' を使用します"
            fi

            echo ""
            print_info "設定するエイリアス:"
            echo -e "  ${CYAN}alias ${ALIAS_UPDATE}='gist-cache-rs update'${NC}"
            echo -e "  ${CYAN}alias ${ALIAS_RUN}='gist-cache-rs run'${NC}"
        fi

        echo ""
        if confirm "これらのエイリアスを $SHELL_RC に追加しますか？" "y"; then
            # 既存のエイリアスをチェック
            UPDATE_EXISTS=false
            RUN_EXISTS=false

            if [ -f "$SHELL_RC" ]; then
                if grep -q "^[[:space:]]*alias[[:space:]]\+${ALIAS_UPDATE}=" "$SHELL_RC"; then
                    UPDATE_EXISTS=true
                fi
                if grep -q "^[[:space:]]*alias[[:space:]]\+${ALIAS_RUN}=" "$SHELL_RC"; then
                    RUN_EXISTS=true
                fi
            fi

            # エイリアスを追加（既存のものはスキップ）
            ADDED=false
            SKIPPED=false

            # マーカーコメントを追加（新規追加がある場合のみ）
            if [ "$UPDATE_EXISTS" = false ] || [ "$RUN_EXISTS" = false ]; then
                echo "" >> "$SHELL_RC"
                echo "# gist-cache-rs aliases: ${ALIAS_UPDATE}, ${ALIAS_RUN} (added on $(date +%Y-%m-%d))" >> "$SHELL_RC"
            fi

            # update エイリアスを追加
            if [ "$UPDATE_EXISTS" = true ]; then
                print_warning "エイリアス '${ALIAS_UPDATE}' は既に存在します（スキップ）"
                SKIPPED=true
            else
                echo "alias ${ALIAS_UPDATE}='gist-cache-rs update'" >> "$SHELL_RC"
                ADDED=true
            fi

            # run エイリアスを追加
            if [ "$RUN_EXISTS" = true ]; then
                print_warning "エイリアス '${ALIAS_RUN}' は既に存在します（スキップ）"
                SKIPPED=true
            else
                echo "alias ${ALIAS_RUN}='gist-cache-rs run'" >> "$SHELL_RC"
                ADDED=true
            fi

            # 結果を表示
            if [ "$ADDED" = true ]; then
                print_success "エイリアスを追加しました"
            fi
            if [ "$SKIPPED" = true ]; then
                print_info "既存のエイリアスは保持されました"
            fi

            if [ "$ADDED" = true ]; then
                print_info "反映するには以下を実行してください:"
                echo -e "  ${CYAN}source $SHELL_RC${NC}"
            fi
        else
            print_info "手動で設定する場合は、上記のエイリアスをシェル設定ファイルに追加してください"
        fi
    else
        print_info "エイリアス設定をスキップしました"
    fi
fi

# ============================================================================
# 完了
# ============================================================================
print_header "セットアップ完了"

print_success "gist-cache-rs のセットアップが完了しました！"
echo ""
echo -e "${BOLD}次のステップ:${NC}"
echo ""
echo "1. コマンドの確認:"
echo -e "   ${CYAN}gist-cache-rs --version${NC}"
echo -e "   ${CYAN}gist-cache-rs --help${NC}"
echo ""
echo "2. キャッシュ更新（まだの場合）:"
echo -e "   ${CYAN}gist-cache-rs update${NC}"
echo ""
echo "3. Gistを検索して実行:"
echo -e "   ${CYAN}gist-cache-rs run --preview keyword${NC}"
echo -e "   ${CYAN}gist-cache-rs run keyword bash${NC}"
echo ""
echo "詳細は以下のドキュメントを参照してください:"
echo "  • README.md - 機能の詳細"
echo "  • docs/QUICKSTART.md - クイックスタートガイド"
echo "  • docs/EXAMPLES.md - 実例集"
echo ""
print_info "問題が発生した場合は、docs/INSTALL.md を確認してください"
echo ""

# Cleanup temporary directory if created
if [ "${CLEANUP_TEMP:-false}" = true ] && [ -n "$TEMP_DIR" ]; then
    print_info "一時ディレクトリをクリーンアップ中..."
    cd "$HOME" || exit 1
    rm -rf "$TEMP_DIR"
    print_success "クリーンアップが完了しました"
    echo ""
fi

exit 0
