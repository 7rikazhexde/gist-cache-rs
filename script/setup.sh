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
# GIST_CACHE_AUTO_ALIAS: true/false (default: false) - Auto-configure aliases in non-interactive mode
# GIST_CACHE_ALIAS_UPDATE: alias name for update (default: gcrsu)
# GIST_CACHE_ALIAS_RUN: alias name for run (default: gcrsr)
# GIST_CACHE_SKIP_CONFIG: true/false (default: false) - Skip config file setup
# GIST_CACHE_AUTO_CONFIG: true/false (default: false) - Auto-configure config in non-interactive mode
# GIST_CACHE_DEFAULT_INTERPRETER: default interpreter (e.g., bash, python3)
INSTALL_METHOD="${GIST_CACHE_INSTALL_METHOD:-1}"
SKIP_CACHE_UPDATE="${GIST_CACHE_SKIP_CACHE:-false}"
SKIP_ALIAS="${GIST_CACHE_SKIP_ALIAS:-false}"
AUTO_ALIAS="${GIST_CACHE_AUTO_ALIAS:-false}"
ALIAS_UPDATE="${GIST_CACHE_ALIAS_UPDATE:-gcrsu}"
ALIAS_RUN="${GIST_CACHE_ALIAS_RUN:-gcrsr}"
SKIP_CONFIG="${GIST_CACHE_SKIP_CONFIG:-false}"
AUTO_CONFIG="${GIST_CACHE_AUTO_CONFIG:-false}"
DEFAULT_INTERPRETER="${GIST_CACHE_DEFAULT_INTERPRETER:-}"

# Functions
print_usage() {
    cat << EOF
Usage: $0 [COMMAND]

COMMAND:
  install     Install (default)
  uninstall   Uninstall
  help        Show this help

Installation examples:
  # Run locally
  ./setup.sh install

  # Run directly with curl (non-interactive mode)
  curl -sSL https://raw.githubusercontent.com/7rikazhexde/gist-cache-rs/main/script/setup.sh | bash

  # Run with curl (customize with environment variables)
  curl -sSL https://raw.githubusercontent.com/7rikazhexde/gist-cache-rs/main/script/setup.sh | GIST_CACHE_INSTALL_METHOD=1 bash

  # Uninstall
  curl -sSL https://raw.githubusercontent.com/7rikazhexde/gist-cache-rs/main/script/setup.sh | bash -s uninstall

Environment variables:
  GIST_CACHE_INSTALL_METHOD       Installation method (1-5, default: 1)
    1: cargo install (recommended)
    2: System directory (/usr/local/bin)
    3: User directory (~/bin)
    4: Symbolic link
    5: Skip
  GIST_CACHE_SKIP_CACHE           Skip cache update (true/false, default: false)
  GIST_CACHE_SKIP_ALIAS           Skip alias configuration (true/false, default: false)
  GIST_CACHE_AUTO_ALIAS           Auto-configure aliases in non-interactive mode (true/false, default: false)
  GIST_CACHE_ALIAS_UPDATE         Alias name for update command (default: gcrsu)
  GIST_CACHE_ALIAS_RUN            Alias name for run command (default: gcrsr)
  GIST_CACHE_SKIP_CONFIG          Skip config file setup (true/false, default: false)
  GIST_CACHE_AUTO_CONFIG          Auto-configure config in non-interactive mode (true/false, default: false)
  GIST_CACHE_DEFAULT_INTERPRETER  Default interpreter (e.g., bash, python3)

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
        print_success "$1 is installed ($(command -v "$1"))"
        return 0
    else
        print_error "$1 not found"
        return 1
    fi
}

check_version() {
    local cmd="$1"
    local version_flag="${2:---version}"

    echo -e "  ${BLUE}Version:${NC} $($cmd "$version_flag" 2>&1 | head -n 1)"
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

    print_warning "This script will uninstall gist-cache-rs"
    echo ""

    # Skip confirmation in non-interactive mode
    if [ "$IS_INTERACTIVE" = true ]; then
        if ! confirm "Start uninstallation?" "n"; then
            echo "Uninstallation cancelled"
            exit 0
        fi
    else
        print_info "Non-interactive mode: Starting uninstallation"
    fi

    print_header "Uninstallation Process"

    UNINSTALLED=false

    # Check and remove from cargo install
    if [ -f "$HOME/.cargo/bin/$BINARY_NAME" ]; then
        print_info "Deleting binary installed with cargo..."
        if cargo uninstall "$BINARY_NAME" 2>/dev/null; then
            print_success "cargo uninstall completed"
            UNINSTALLED=true
        else
            print_warning "cargo uninstall failed (attempting manual deletion)"
            if rm -f "$HOME/.cargo/bin/$BINARY_NAME"; then
                print_success "$HOME/.cargo/bin/$BINARY_NAME deleted"
                UNINSTALLED=true
            fi
        fi
    fi

    # Check and remove from /usr/local/bin
    if [ -f "/usr/local/bin/$BINARY_NAME" ]; then
        print_info "Deleting binary from /usr/local/bin..."
        if sudo rm -f "/usr/local/bin/$BINARY_NAME"; then
            print_success "/usr/local/bin/$BINARY_NAME deleted"
            UNINSTALLED=true
        else
            print_error "Deletion failed (permissions required)"
        fi
    fi

    # Check and remove from ~/bin
    if [ -f "$HOME/bin/$BINARY_NAME" ]; then
        print_info "Deleting binary from $HOME/bin..."
        if rm -f "$HOME/bin/$BINARY_NAME"; then
            print_success "$HOME/bin/$BINARY_NAME deleted"
            UNINSTALLED=true
        fi
    fi

    if [ "$UNINSTALLED" = false ]; then
        print_warning "No installation of $BINARY_NAME found"
    fi

    # Ask about cache directory
    echo ""
    if [ -d "$CACHE_DIR" ]; then
        print_info "Cache directory detected: $CACHE_DIR"

        SHOULD_DELETE_CACHE=false
        if [ "$IS_INTERACTIVE" = false ]; then
            # Delete in non-interactive mode
            print_info "Non-interactive mode: Deleting cache directory"
            SHOULD_DELETE_CACHE=true
        elif confirm "Delete cache directory?" "n"; then
            SHOULD_DELETE_CACHE=true
        fi

        if [ "$SHOULD_DELETE_CACHE" = true ]; then
            if rm -rf "$CACHE_DIR"; then
                print_success "Cache directory deleted"
            else
                print_error "Failed to delete cache directory"
            fi
        else
            print_info "Cache directory retained"
        fi
    fi

    # Ask about config directory
    echo ""
    CONFIG_DIR="$HOME/.config/gist-cache"
    if [ -d "$CONFIG_DIR" ]; then
        print_info "Configuration directory detected: $CONFIG_DIR"

        SHOULD_DELETE_CONFIG=false
        if [ "$IS_INTERACTIVE" = false ]; then
            # Delete in non-interactive mode
            print_info "Non-interactive mode: Deleting configuration directory"
            SHOULD_DELETE_CONFIG=true
        elif confirm "Delete configuration directory?" "n"; then
            SHOULD_DELETE_CONFIG=true
        fi

        if [ "$SHOULD_DELETE_CONFIG" = true ]; then
            if rm -rf "$CONFIG_DIR"; then
                print_success "Configuration directory deleted"
            else
                print_error "Failed to delete configuration directory"
            fi
        else
            print_info "Configuration directory retained"
        fi
    fi

    # Ask about aliases
    echo ""
    print_info "Checking alias settings"

    for rcfile in "$HOME/.bashrc" "$HOME/.zshrc" "$HOME/.config/fish/config.fish"; do
        if [ ! -f "$rcfile" ]; then
            continue
        fi

        # Detect gist-cache-rs related aliases (search by command content)
        FOUND_ALIASES=()

        # Search for aliases containing 'gist-cache-rs update'
        while IFS= read -r line; do
            if [[ "$line" =~ ^[[:space:]]*alias[[:space:]]+([^=]+)=[[:space:]]*[\'\"]*gist-cache-rs[[:space:]]+update ]]; then
                ALIAS_NAME="${BASH_REMATCH[1]}"
                FOUND_ALIASES+=("${ALIAS_NAME}:update")
            fi
        done < "$rcfile"

        # Search for aliases containing 'gist-cache-rs run'
        while IFS= read -r line; do
            if [[ "$line" =~ ^[[:space:]]*alias[[:space:]]+([^=]+)=[[:space:]]*[\'\"]*gist-cache-rs[[:space:]]+run ]]; then
                ALIAS_NAME="${BASH_REMATCH[1]}"
                FOUND_ALIASES+=("${ALIAS_NAME}:run")
            fi
        done < "$rcfile"

        # If aliases are found
        if [ ${#FOUND_ALIASES[@]} -gt 0 ]; then
            print_warning "Alias settings remain in $rcfile"

            echo "  Detected aliases:"
            for alias_entry in "${FOUND_ALIASES[@]}"; do
                ALIAS_NAME="${alias_entry%%:*}"
                ALIAS_TYPE="${alias_entry##*:}"
                if [ "$ALIAS_TYPE" = "update" ]; then
                    echo "    - ${ALIAS_NAME} -> gist-cache-rs update"
                else
                    echo "    - ${ALIAS_NAME} -> gist-cache-rs run"
                fi
            done
            echo ""

            SHOULD_DELETE_ALIAS=false
            if [ "$IS_INTERACTIVE" = false ]; then
                # Delete in non-interactive mode
                print_info "Non-interactive mode: Deleting aliases"
                SHOULD_DELETE_ALIAS=true
            elif confirm "Delete aliases from $rcfile?" "n"; then
                SHOULD_DELETE_ALIAS=true
            fi

            if [ "$SHOULD_DELETE_ALIAS" = true ]; then
                # Create backup
                BACKUP_FILE="${rcfile}.backup.$(date +%Y%m%d%H%M%S)"
                cp "$rcfile" "$BACKUP_FILE"

                # Remove marker comments (supports multiple patterns)
                sed -i.tmp '/# gist-cache-rs aliases/d' "$rcfile"

                # Delete each detected alias
                for alias_entry in "${FOUND_ALIASES[@]}"; do
                    ALIAS_NAME="${alias_entry%%:*}"
                    # Escape alias name (for special characters)
                    ESCAPED_ALIAS=$(printf '%s\n' "$ALIAS_NAME" | sed 's/[]\/$*.^[]/\\&/g')
                    sed -i.tmp "/^[[:space:]]*alias[[:space:]]\+${ESCAPED_ALIAS}=/d" "$rcfile"
                done

                # Delete temporary file
                rm -f "${rcfile}.tmp"

                print_success "Aliases deleted (backup: $BACKUP_FILE)"
            fi
        fi
    done

    print_header "Uninstallation Complete"
    print_success "gist-cache-rs uninstallation complete"
    echo ""
    print_info "Thank you for using!"
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
        print_error "Invalid command: $MODE"
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

print_info "This script will set up gist-cache-rs"

# Non-interactive mode notification
if [ "$IS_INTERACTIVE" = false ]; then
    echo ""
    print_warning "Running in non-interactive mode (using default settings)"
    print_info "Set environment variables to customize"
    print_info "Details: $0 help"
    echo ""
else
    echo ""
    if ! confirm "Start setup?" "y"; then
        echo "Setup cancelled"
        exit 0
    fi
fi

# ============================================================================
# Step 1: Prerequisites Check
# ============================================================================
print_header "Step 1: Prerequisites Check"

PREREQUISITES_OK=true

# Check Rust
echo -e "${BOLD}Rust Toolchain:${NC}"
if check_command "rustc" && check_command "cargo"; then
    check_version "rustc" "--version"
    check_version "cargo" "--version"

    # Check minimum version (1.85)
    RUST_VERSION=$(rustc --version | grep -oP '\d+\.\d+' | head -1)
    if [ "$(echo "$RUST_VERSION >= 1.85" | bc -l 2>/dev/null || echo 0)" -eq 1 ] 2>/dev/null; then
        print_success "Rust version meets requirements (>= 1.85)"
    else
        print_warning "Rust version may be old (recommended: 1.85 or later)"
    fi
else
    print_error "Rust is not installed"
    print_info "Installation method: https://rustup.rs/"
    print_info "Command: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    PREREQUISITES_OK=false
fi

echo ""

# Check GitHub CLI
echo -e "${BOLD}GitHub CLI:${NC}"
if check_command "gh"; then
    check_version "gh" "--version"

    # Check authentication
    if gh auth status &> /dev/null; then
        print_success "GitHub CLI is authenticated"
        GH_USER=$(gh api user --jq .login 2>/dev/null || echo "unknown")
        echo -e "  ${BLUE}User:${NC} $GH_USER"
    else
        print_error "GitHub CLI is not authenticated"
        print_info "Authentication command: gh auth login"
        PREREQUISITES_OK=false
    fi
else
    print_error "GitHub CLI (gh) is not installed"
    print_info "Installation method: https://cli.github.com/"
    PREREQUISITES_OK=false
fi

echo ""

if [ "$PREREQUISITES_OK" = false ]; then
    print_error "Prerequisites not met"
    echo ""
    if [ "$IS_INTERACTIVE" = true ]; then
        if confirm "Continue anyway?" "n"; then
            print_warning "Continuing with insufficient prerequisites"
        else
            echo "Setup cancelled"
            exit 1
        fi
    else
        print_error "Prerequisites are mandatory in non-interactive mode"
        exit 1
    fi
fi

print_success "Prerequisites Check Complete"

# ============================================================================
# Step 2: Project Directory Confirmation
# ============================================================================
print_header "Step 2: Project Directory Confirmation"

# Check if Cargo.toml exists
if [ -f "$SCRIPT_DIR/../Cargo.toml" ]; then
    print_success "Project directory detected: $(dirname "$SCRIPT_DIR")"
    PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
elif [ -f "$SCRIPT_DIR/Cargo.toml" ]; then
    print_success "Project directory detected: $SCRIPT_DIR"
    PROJECT_DIR="$SCRIPT_DIR"
elif [ -f "./Cargo.toml" ]; then
    print_success "Project directory detected: $(pwd)"
    PROJECT_DIR="$(pwd)"
else
    print_warning "Cargo.toml not found"
    echo ""

    # Non-interactive mode: automatically clone
    if [ "$IS_INTERACTIVE" = false ]; then
        print_info "Non-interactive mode: Automatically cloning repository"
        SHOULD_CLONE=true
    else
        if confirm "Clone project from GitHub?" "y"; then
            SHOULD_CLONE=true
        else
            SHOULD_CLONE=false
        fi
    fi

    if [ "$SHOULD_CLONE" = true ]; then
        # Check if git is available
        if ! command -v git &> /dev/null; then
            print_error "git command not found"
            print_info "Please install git and try again"
            exit 1
        fi

        # Create temp directory
        TEMP_DIR=$(mktemp -d)
        print_info "Temporary directory: $TEMP_DIR"

        # Clone repository
        echo ""
        print_info "Cloning repository..."
        if git clone "$REPO_URL" "$TEMP_DIR/gist-cache-rs"; then
            print_success "Clone complete"
            PROJECT_DIR="$TEMP_DIR/gist-cache-rs"
            CLEANUP_TEMP=true
        else
            print_error "Clone failed"
            rm -rf "$TEMP_DIR"
            exit 1
        fi
    else
        echo ""
        read -r -p "$(echo -e "${YELLOW}Please enter the path to the project directory: ${NC}")" PROJECT_DIR

        if [ ! -f "$PROJECT_DIR/Cargo.toml" ]; then
            print_error "Cargo.toml not found in the specified directory"
            exit 1
        fi
    fi
fi

cd "$PROJECT_DIR" || exit 1
print_info "Working directory: $(pwd)"


# ============================================================================
# Step 3: Build
# ============================================================================
print_header "Step 3: Build"

if [ "$IS_INTERACTIVE" = false ]; then
    print_info "Non-interactive mode: Automatically performing release build"
    SHOULD_BUILD=true
else
    if confirm "Perform release build?" "y"; then
        SHOULD_BUILD=true
    else
        SHOULD_BUILD=false
    fi
fi

if [ "$SHOULD_BUILD" = true ]; then
    echo ""
    print_info "Starting build (may take some time)..."
    echo ""

    if cargo build --release; then
        print_success "Build complete"

        # Display binary size
        if [ -f "target/release/gist-cache-rs" ]; then
            BINARY_SIZE=$(du -h target/release/gist-cache-rs | cut -f1)
            print_info "Binary size: $BINARY_SIZE"
        fi
    else
        print_error "Build failed"
        exit 1
    fi
else
    print_warning "Build skipped"

    if [ ! -f "target/release/gist-cache-rs" ]; then
        print_error "Built binary not found"
        exit 1
    fi
fi

# ============================================================================
# Step 4: Installation
# ============================================================================
print_header "Step 4: Installation"

if [ "$IS_INTERACTIVE" = false ]; then
    print_info "Non-interactive mode: Using installation method ${INSTALL_METHOD}"
    INSTALL_CHOICE="$INSTALL_METHOD"
else
    echo "Select installation method:"
    echo "  1) cargo install (recommended) - Install to ~/.cargo/bin"
    echo "  2) System directory - Copy to /usr/local/bin (sudo required)"
    echo "  3) User directory - Copy to ~/bin"
    echo "  4) Symbolic link - For developers"
    echo "  5) Skip"
    echo ""

    read -r -p "$(echo -e "${YELLOW}Selection [1-5]: ${NC}")" INSTALL_CHOICE

    # Validate input
    if [[ ! "$INSTALL_CHOICE" =~ ^[1-5]$ ]]; then
        print_error "Invalid selection"
        exit 1
    fi
fi

case $INSTALL_CHOICE in
    1)
        print_info "Performing cargo install..."
        if cargo install --path .; then
            print_success "Installation complete"
            print_info "Installation destination: ~/.cargo/bin/gist-cache-rs"

            # PATH check
            if echo "$PATH" | grep -q ".cargo/bin"; then
                print_success "$HOME/.cargo/bin is included in PATH"
            else
                print_warning "$HOME/.cargo/bin is not included in PATH"
                print_info "Add the following to ~/.bashrc or ~/.zshrc:"
                echo -e "  ${CYAN}export PATH=\"\$HOME/.cargo/bin:\$PATH\"${NC}"
            fi
        else
            print_error "Installation failed"
            exit 1
        fi
        ;;
    2)
        print_info "Copying to system directory..."
        if sudo cp target/release/gist-cache-rs /usr/local/bin/; then
            print_success "Installation complete"
            print_info "Installation destination: /usr/local/bin/gist-cache-rs"
        else
            print_error "Installation failed"
            exit 1
        fi
        ;;
    3)
        print_info "Copying to user directory..."
        mkdir -p ~/bin
        if cp target/release/gist-cache-rs ~/bin/; then
            print_success "Installation complete"
            print_info "Installation destination: ~/bin/gist-cache-rs"

            # PATH check
            if echo "$PATH" | grep -q "$HOME/bin"; then
                print_success "$HOME/bin is included in PATH"
            else
                print_warning "$HOME/bin is not included in PATH"
                print_info "Add the following to ~/.bashrc or ~/.zshrc:"
                echo -e "  ${CYAN}export PATH=\"\$HOME/bin:\$PATH\"${NC}"
            fi
        else
            print_error "Installation failed"
            exit 1
        fi
        ;;
    4)
        print_info "Creating symbolic link..."

        if [ "$IS_INTERACTIVE" = false ]; then
            # Non-interactive: default to ~/bin
            LINK_CHOICE=2
        else
            echo "  1) /usr/local/bin (sudo required)"
            echo "  2) ~/bin"
            read -r -p "$(echo -e "${YELLOW}Selection [1-2]: ${NC}")" LINK_CHOICE
        fi

        case $LINK_CHOICE in
            1)
                if sudo ln -sf "$(pwd)/target/release/gist-cache-rs" /usr/local/bin/gist-cache-rs; then
                    print_success "Symbolic link created"
                    print_info "Link destination: /usr/local/bin/gist-cache-rs"
                fi
                ;;
            2)
                mkdir -p ~/bin
                if ln -sf "$(pwd)/target/release/gist-cache-rs" ~/bin/gist-cache-rs; then
                    print_success "Symbolic link created"
                    print_info "Link destination: ~/bin/gist-cache-rs"
                fi
                ;;
        esac
        ;;
    5)
        print_warning "Installation skipped"
        ;;
    *)
        print_error "Invalid selection"
        exit 1
        ;;
esac

# ============================================================================
# Step 5: Installation Confirmation
# ============================================================================
print_header "Step 5: Installation Confirmation"

if command -v gist-cache-rs &> /dev/null; then
    print_success "gist-cache-rs command is available"
    check_version "gist-cache-rs" "--version"
    print_info "Path: $(which gist-cache-rs)"
else
    print_warning "gist-cache-rs command not found"
    print_info "Please restart your shell or update your PATH"
fi

# ============================================================================
# Step 6: Initial Cache Creation
# ============================================================================
print_header "Step 6: Initial Cache Creation"

if [ "$SKIP_CACHE_UPDATE" = "true" ]; then
    print_info "Skipping cache update due to environment variable"
elif ! command -v gist-cache-rs &> /dev/null; then
    print_warning "Skipping cache creation because command is unavailable"
    print_info "Please run 'gist-cache-rs update' later"
else
    if [ "$IS_INTERACTIVE" = false ]; then
        print_info "Non-interactive mode: Automatically performing cache update"
        SHOULD_UPDATE=true
    else
        if confirm "Perform initial cache update?" "y"; then
            SHOULD_UPDATE=true
        else
            SHOULD_UPDATE=false
        fi
    fi

    if [ "$SHOULD_UPDATE" = true ]; then
        echo ""
        print_info "Starting cache update..."
        echo ""

        if gist-cache-rs update --verbose; then
            print_success "Cache update complete"
        else
            print_error "Cache update failed"
        fi
    else
        print_info "Please run 'gist-cache-rs update' later"
    fi
fi

# ============================================================================
# Step 6.5: Configuration File Setup (Optional)
# ============================================================================
print_header "Step 6.5: Configuration File Setup (Optional)"

if [ "$SKIP_CONFIG" = "true" ]; then
    print_info "Skipping config file setup due to environment variable"
elif ! command -v gist-cache-rs &> /dev/null; then
    print_warning "Skipping config setup because command is unavailable"
else
    CONFIG_DIR="$HOME/.config/gist-cache"
    CONFIG_FILE="$CONFIG_DIR/config.toml"

    # Check if config file already exists
    if [ -f "$CONFIG_FILE" ]; then
        print_success "Configuration file already exists: $CONFIG_FILE"

        if [ "$IS_INTERACTIVE" = true ]; then
            if confirm "View current configuration?" "y"; then
                echo ""
                gist-cache-rs config show
                echo ""
            fi

            # Ask if user wants to update existing config
            if confirm "Update configuration?" "n"; then
                SHOULD_SETUP_CONFIG=true
            else
                print_info "Configuration retained without changes"
                SHOULD_SETUP_CONFIG=false
            fi
        else
            # Non-interactive mode: skip updating existing config
            print_info "Configuration file exists. Skipping update in non-interactive mode"
            SHOULD_SETUP_CONFIG=false
        fi
    else
        SHOULD_SETUP_CONFIG=false

        if [ "$IS_INTERACTIVE" = false ]; then
            # Non-interactive mode
            if [ "$AUTO_CONFIG" = "true" ]; then
                print_info "Non-interactive mode: Performing automatic config setup"
                SHOULD_SETUP_CONFIG=true
            else
                print_info "Non-interactive mode: Skipping config setup"
                print_info "To configure later, run: gist-cache-rs config set <key> <value>"
            fi
        elif confirm "Set up default configuration?" "y"; then
            SHOULD_SETUP_CONFIG=true
        fi

        if [ "$SHOULD_SETUP_CONFIG" = true ]; then
            echo ""
            print_info "Configuring default settings..."
            echo ""

            # Create config directory if it doesn't exist
            mkdir -p "$CONFIG_DIR"

            # Configure interpreters for each extension
            CONFIG_APPLIED=false

            if [ "$IS_INTERACTIVE" = true ]; then
                echo "Configure interpreters for each file extension:"
                echo -e "${CYAN}(Press Enter to skip any extension)${NC}"
                echo ""

                # Python (.py)
                echo -e "${BOLD}Python files (.py):${NC}"
                echo "  1) uv (recommended for modern Python)"
                echo "  2) python3"
                echo "  3) Skip"
                read -r -p "$(echo -e "${YELLOW}Selection [1-3]: ${NC}")" PY_CHOICE
                case $PY_CHOICE in
                    1)
                        if gist-cache-rs config set defaults.interpreter.py uv 2>/dev/null; then
                            print_success "Python (.py) → uv"
                            CONFIG_APPLIED=true
                        fi
                        ;;
                    2)
                        if gist-cache-rs config set defaults.interpreter.py python3 2>/dev/null; then
                            print_success "Python (.py) → python3"
                            CONFIG_APPLIED=true
                        fi
                        ;;
                esac
                echo ""

                # Ruby (.rb)
                if confirm "Configure Ruby interpreter for .rb files?" "n"; then
                    if gist-cache-rs config set defaults.interpreter.rb ruby 2>/dev/null; then
                        print_success "Ruby (.rb) → ruby"
                        CONFIG_APPLIED=true
                    fi
                fi
                echo ""

                # JavaScript (.js)
                if confirm "Configure Node.js interpreter for .js files?" "n"; then
                    if gist-cache-rs config set defaults.interpreter.js node 2>/dev/null; then
                        print_success "JavaScript (.js) → node"
                        CONFIG_APPLIED=true
                    fi
                fi
                echo ""

                # TypeScript (.ts)
                echo -e "${BOLD}TypeScript files (.ts):${NC}"
                echo "  1) ts-node"
                echo "  2) deno"
                echo "  3) bun"
                echo "  4) Skip"
                read -r -p "$(echo -e "${YELLOW}Selection [1-4]: ${NC}")" TS_CHOICE
                case $TS_CHOICE in
                    1)
                        if gist-cache-rs config set defaults.interpreter.ts ts-node 2>/dev/null; then
                            print_success "TypeScript (.ts) → ts-node"
                            CONFIG_APPLIED=true
                        fi
                        ;;
                    2)
                        if gist-cache-rs config set defaults.interpreter.ts deno 2>/dev/null; then
                            print_success "TypeScript (.ts) → deno"
                            CONFIG_APPLIED=true
                        fi
                        ;;
                    3)
                        if gist-cache-rs config set defaults.interpreter.ts bun 2>/dev/null; then
                            print_success "TypeScript (.ts) → bun"
                            CONFIG_APPLIED=true
                        fi
                        ;;
                esac
                echo ""

                # Shell scripts (.sh)
                echo -e "${BOLD}Shell scripts (.sh):${NC}"
                echo "  1) bash (recommended)"
                echo "  2) sh"
                echo "  3) zsh"
                echo "  4) Skip"
                read -r -p "$(echo -e "${YELLOW}Selection [1-4]: ${NC}")" SH_CHOICE
                case $SH_CHOICE in
                    1)
                        if gist-cache-rs config set defaults.interpreter.sh bash 2>/dev/null; then
                            print_success "Shell (.sh) → bash"
                            CONFIG_APPLIED=true
                        fi
                        ;;
                    2)
                        if gist-cache-rs config set defaults.interpreter.sh sh 2>/dev/null; then
                            print_success "Shell (.sh) → sh"
                            CONFIG_APPLIED=true
                        fi
                        ;;
                    3)
                        if gist-cache-rs config set defaults.interpreter.sh sh 2>/dev/null; then
                            print_success "Shell (.sh) → zsh"
                            CONFIG_APPLIED=true
                        fi
                        ;;
                esac
                echo ""

                # PHP (.php)
                if confirm "Configure PHP interpreter for .php files?" "n"; then
                    if gist-cache-rs config set defaults.interpreter.php php 2>/dev/null; then
                        print_success "PHP (.php) → php"
                        CONFIG_APPLIED=true
                    fi
                fi
                echo ""

                # Perl (.pl)
                if confirm "Configure Perl interpreter for .pl files?" "n"; then
                    if gist-cache-rs config set defaults.interpreter.pl perl 2>/dev/null; then
                        print_success "Perl (.pl) → perl"
                        CONFIG_APPLIED=true
                    fi
                fi
                echo ""

                # PowerShell (.ps1)
                if confirm "Configure PowerShell interpreter for .ps1 files?" "n"; then
                    if gist-cache-rs config set defaults.interpreter.ps1 pwsh 2>/dev/null; then
                        print_success "PowerShell (.ps1) → pwsh"
                        CONFIG_APPLIED=true
                    fi
                fi
                echo ""

                # Wildcard fallback (*)
                echo -e "${BOLD}Fallback interpreter (for unrecognized extensions):${NC}"
                echo "  1) bash (recommended)"
                echo "  2) python3"
                echo "  3) sh"
                echo "  4) Skip"
                read -r -p "$(echo -e "${YELLOW}Selection [1-4]: ${NC}")" FALLBACK_CHOICE
                case $FALLBACK_CHOICE in
                    1)
                        if gist-cache-rs config set defaults.interpreter."*" bash 2>/dev/null; then
                            print_success "Fallback (*) → bash"
                            CONFIG_APPLIED=true
                        fi
                        ;;
                    2)
                        if gist-cache-rs config set defaults.interpreter."*" python3 2>/dev/null; then
                            print_success "Fallback (*) → python3"
                            CONFIG_APPLIED=true
                        fi
                        ;;
                    3)
                        if gist-cache-rs config set defaults.interpreter."*" sh 2>/dev/null; then
                            print_success "Fallback (*) → sh"
                            CONFIG_APPLIED=true
                        fi
                        ;;
                esac
                echo ""

            elif [ -n "$DEFAULT_INTERPRETER" ]; then
                # Non-interactive mode: use environment variable for wildcard only
                if gist-cache-rs config set defaults.interpreter."*" "$DEFAULT_INTERPRETER" 2>/dev/null; then
                    print_success "Fallback (*) → $DEFAULT_INTERPRETER"
                    CONFIG_APPLIED=true
                fi
            fi

            # Additional configuration options (interactive only)
            if [ "$IS_INTERACTIVE" = true ]; then
                if confirm "Configure execution confirmation (safety feature)?" "n"; then
                    if gist-cache-rs config set execution.confirm_before_run true 2>/dev/null; then
                        print_success "Execution confirmation enabled"
                        CONFIG_APPLIED=true
                    fi
                fi
            fi

            if [ "$CONFIG_APPLIED" = false ]; then
                print_warning "No configuration was applied"
            fi

            echo ""
            print_success "Configuration setup complete"

            if [ -f "$CONFIG_FILE" ]; then
                print_info "Configuration file: $CONFIG_FILE"

                if [ "$IS_INTERACTIVE" = true ]; then
                    if confirm "View configuration?" "y"; then
                        echo ""
                        gist-cache-rs config show
                    fi
                fi
            fi

            echo ""
            print_info "To modify configuration later, use:"
            echo -e "  ${CYAN}gist-cache-rs config set <key> <value>${NC}"
            echo -e "  ${CYAN}gist-cache-rs config show${NC}"
        else
            print_info "Configuration setup skipped"
            print_info "To configure later, use: gist-cache-rs config set <key> <value>"
        fi
    fi
fi

# ============================================================================
# Step 7: Alias Configuration (Optional)
# ============================================================================
print_header "Step 7: Alias Configuration (Optional)"

if [ "$SKIP_ALIAS" = "true" ]; then
    print_info "Skipping alias configuration due to environment variable"
else
    if [ "$IS_INTERACTIVE" = false ]; then
        # Non-interactive mode
        if [ "$AUTO_ALIAS" = "true" ]; then
            # Execute automatic configuration
            print_info "Non-interactive mode: Performing automatic alias configuration"
            echo ""

            # Detect shell (judging from SHELL environment variable)
            if [[ "$SHELL" == *"zsh"* ]]; then
                SHELL_RC="$HOME/.zshrc"
            elif [[ "$SHELL" == *"bash"* ]]; then
                SHELL_RC="$HOME/.bashrc"
            else
                SHELL_RC="$HOME/.bashrc"
            fi

            print_info "Configuration file: $SHELL_RC"
            print_info "Aliases to be set:"
            echo -e "  ${CYAN}alias ${ALIAS_UPDATE}='gist-cache-rs update'${NC}"
            echo -e "  ${CYAN}alias ${ALIAS_RUN}='gist-cache-rs run'${NC}"
            echo ""

            # Check existing aliases
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

            # Add aliases (skip existing ones)
            ADDED=false
            SKIPPED=false

            # Add marker comment (only if new additions)
            if [ "$UPDATE_EXISTS" = false ] || [ "$RUN_EXISTS" = false ]; then
                echo "" >> "$SHELL_RC"
                echo "# gist-cache-rs aliases: ${ALIAS_UPDATE}, ${ALIAS_RUN} (added on $(date +%Y-%m-%d))" >> "$SHELL_RC"
            fi

            # Add update alias
            if [ "$UPDATE_EXISTS" = true ]; then
                print_warning "Alias '${ALIAS_UPDATE}' already exists (skipped)"
                SKIPPED=true
            else
                echo "alias ${ALIAS_UPDATE}='gist-cache-rs update'" >> "$SHELL_RC"
                ADDED=true
            fi

            # Add run alias
            if [ "$RUN_EXISTS" = true ]; then
                print_warning "Alias '${ALIAS_RUN}' already exists (skipped)"
                SKIPPED=true
            else
                echo "alias ${ALIAS_RUN}='gist-cache-rs run'" >> "$SHELL_RC"
                ADDED=true
            fi

            # Display results
            if [ "$ADDED" = true ]; then
                print_success "Aliases added"
                print_info "To reflect changes, run:"
                echo -e "  ${CYAN}source $SHELL_RC${NC}"
            fi
            if [ "$SKIPPED" = true ]; then
                print_info "Existing aliases retained"
            fi
        else
            # Guide manual configuration if not auto-configured
            print_info "Non-interactive mode: Skipping alias configuration"
            print_info "If configuring manually:"
            echo -e "  ${CYAN}alias gcrsu='gist-cache-rs update'${NC}"
            echo -e "  ${CYAN}alias gcrsr='gist-cache-rs run'${NC}"
            echo ""
            print_info "Or, for automatic configuration with environment variables:"
            echo -e "  ${CYAN}GIST_CACHE_AUTO_ALIAS=true${NC}"
        fi
    elif confirm "Configure convenient aliases?" "y"; then
        echo ""
        echo "Recommended aliases:"
        echo -e "  ${CYAN}alias gcrsu='gist-cache-rs update'${NC}"
        echo -e "  ${CYAN}alias gcrsr='gist-cache-rs run'${NC}"
        echo ""

        # Detect shell
        if [ -n "$BASH_VERSION" ]; then
            SHELL_RC="$HOME/.bashrc"
        elif [ -n "$ZSH_VERSION" ]; then
            SHELL_RC="$HOME/.zshrc"
        else
            SHELL_RC="$HOME/.bashrc"
        fi

        print_info "Configuration file: $SHELL_RC"
        echo ""

        # Confirm use of recommended aliases
        if confirm "Use recommended alias names (gcrsu, gcrsr)?" "y"; then
            ALIAS_UPDATE="gcrsu"
            ALIAS_RUN="gcrsr"
        else
            # Enter custom alias names
            echo ""
            print_info "Enter custom alias names"
            echo ""

            read -r -p "$(echo -e "${YELLOW}Alias name for gist-cache-rs update: ${NC}")" ALIAS_UPDATE
            if [ -z "$ALIAS_UPDATE" ]; then
                ALIAS_UPDATE="gcrsu"
                print_warning "No input, using default name 'gcrsu'"
            fi

            read -r -p "$(echo -e "${YELLOW}Alias name for gist-cache-rs run: ${NC}")" ALIAS_RUN
            if [ -z "$ALIAS_RUN" ]; then
                ALIAS_RUN="gcrsr"
                print_warning "No input, using default name 'gcrsr'"
            fi

            echo ""
            print_info "Aliases to be set:"
            echo -e "  ${CYAN}alias ${ALIAS_UPDATE}='gist-cache-rs update'${NC}"
            echo -e "  ${CYAN}alias ${ALIAS_RUN}='gist-cache-rs run'${NC}"
        fi

        echo ""
        if confirm "Add these aliases to $SHELL_RC?" "y"; then
            # Check existing aliases
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

            # Add aliases (skip existing ones)
            ADDED=false
            SKIPPED=false

            # Add marker comment (only if new additions)
            if [ "$UPDATE_EXISTS" = false ] || [ "$RUN_EXISTS" = false ]; then
                echo "" >> "$SHELL_RC"
                echo "# gist-cache-rs aliases: ${ALIAS_UPDATE}, ${ALIAS_RUN} (added on $(date +%Y-%m-%d))" >> "$SHELL_RC"
            fi

            # Add update alias
            if [ "$UPDATE_EXISTS" = true ]; then
                print_warning "Alias '${ALIAS_UPDATE}' already exists (skipped)"
                SKIPPED=true
            else
                echo "alias ${ALIAS_UPDATE}='gist-cache-rs update'" >> "$SHELL_RC"
                ADDED=true
            fi

            # Add run alias
            if [ "$RUN_EXISTS" = true ]; then
                print_warning "Alias '${ALIAS_RUN}' already exists (skipped)"
                SKIPPED=true
            else
                echo "alias ${ALIAS_RUN}='gist-cache-rs run'" >> "$SHELL_RC"
                ADDED=true
            fi

            # Display results
            if [ "$ADDED" = true ]; then
                print_success "Aliases added"
            fi
            if [ "$SKIPPED" = true ]; then
                print_info "Existing aliases retained"
            fi

            if [ "$ADDED" = true ]; then
                print_info "To reflect changes, run:"
                echo -e "  ${CYAN}source $SHELL_RC${NC}"
            fi
        else
            print_info "If configuring manually, add the above aliases to your shell configuration file"
        fi
    else
        print_info "Alias configuration skipped"
    fi
fi

# ============================================================================
# Step 8: Shell Completion Setup (Optional)
# ============================================================================
print_header "Step 8: Shell Completion Setup (Optional)"

if ! command -v gist-cache-rs &> /dev/null; then
    print_warning "Skipping completion setup because gist-cache-rs command is unavailable"
else
    if [ "$IS_INTERACTIVE" = false ]; then
        print_info "Skipping interactive completion setup."
        print_info "To enable, run 'gist-cache-rs completions <your-shell>' and follow instructions."
    elif confirm "Configure shell completion?" "y"; then

        DETECTED_SHELL=""
        if [[ "$SHELL" == *"/zsh"* ]]; then
            DETECTED_SHELL="zsh"
        elif [[ "$SHELL" == *"/bash"* ]]; then
            DETECTED_SHELL="bash"
        elif [[ "$SHELL" == *"/fish"* ]]; then
            DETECTED_SHELL="fish"
        fi

        if [ -z "$DETECTED_SHELL" ]; then
            print_warning "Could not detect your shell. Skipping completion setup."
        else
            print_info "Detected shell: $DETECTED_SHELL"
            echo ""

            # Bash
            if [ "$DETECTED_SHELL" == "bash" ]; then
                RC_FILE="$HOME/.bashrc"
                COMPLETION_DIR="$HOME/.local/share/bash-completion/completions"
                COMPLETION_PATH="$COMPLETION_DIR/gist-cache-rs"

                print_info "Configuring for Bash..."

                # Check if already configured
                if [ -f "$COMPLETION_PATH" ]; then
                    print_success "Bash completion seems to be already configured. Updating script..."
                    gist-cache-rs completions bash > "$COMPLETION_PATH"
                    print_success "Completion script updated at $COMPLETION_PATH"
                else
                    if confirm "Proceed with Bash completion setup?" "y"; then
                        mkdir -p "$COMPLETION_DIR"
                        gist-cache-rs completions bash > "$COMPLETION_PATH"
                        print_success "Completion script created at $COMPLETION_PATH"

                        echo ""
                        print_info "Checking if manual .bashrc configuration is needed..."
                        print_warning "If completion doesn't work after restarting your shell, you may need to add the following line to your $RC_FILE:"
                        echo -e "  ${CYAN}source $COMPLETION_PATH${NC}"
                    fi
                fi
            fi

            # Zsh
            if [ "$DETECTED_SHELL" == "zsh" ]; then
                RC_FILE="$HOME/.zshrc"
                COMPLETION_DIR="$HOME/.zfunc"
                COMPLETION_PATH="$COMPLETION_DIR/_gist-cache-rs"

                print_info "Configuring for Zsh..."

                if [ -f "$COMPLETION_PATH" ]; then
                     print_success "Zsh completion seems to be already configured. Updating script..."
                     gist-cache-rs completions zsh > "$COMPLETION_PATH"
                     print_success "Completion script updated at $COMPLETION_PATH"
                else
                    if confirm "Proceed with Zsh completion setup?" "y"; then
                        mkdir -p "$COMPLETION_DIR"
                        gist-cache-rs completions zsh > "$COMPLETION_PATH"
                        print_success "Completion script created at $COMPLETION_PATH"

                        if ! grep -q "fpath=($COMPLETION_DIR" "$RC_FILE" 2>/dev/null; then
                            echo -e "\n# For gist-cache-rs completion\nfpath=($COMPLETION_DIR \$fpath)" >> "$RC_FILE"
                            print_info "Added '$COMPLETION_DIR' to fpath in $RC_FILE"
                        fi
                        if ! grep -q "compinit" "$RC_FILE" 2>/dev/null; then
                           echo -e "\n# Initialize completion system\nautoload -Uz compinit && compinit" >> "$RC_FILE"
                           print_info "Added 'compinit' to $RC_FILE"
                        fi
                    fi
                fi
            fi

            # Fish
            if [ "$DETECTED_SHELL" == "fish" ]; then
                COMPLETION_PATH="$HOME/.config/fish/completions/gist-cache-rs.fish"
                print_info "Configuring for Fish..."

                if confirm "Proceed with Fish completion setup?" "y"; then
                    mkdir -p "$(dirname "$COMPLETION_PATH")"
                    gist-cache-rs completions fish > "$COMPLETION_PATH"
                    print_success "Completion script created/updated at $COMPLETION_PATH"
                fi
            fi

            echo ""
            print_info "Setup for $DETECTED_SHELL completion is complete."
            print_warning "Please restart your shell or reload your configuration (e.g., 'source ~/.bashrc') to activate."
        fi
    fi
fi

# ============================================================================
# Complete
# ============================================================================
print_header "Setup Complete"

print_success "gist-cache-rs setup is complete!"
echo ""
echo -e "${BOLD}Next Steps:${NC}"
echo ""
echo "1. Command verification:"
echo -e "   ${CYAN}gist-cache-rs --version${NC}"
echo -e "   ${CYAN}gist-cache-rs --help${NC}"
echo ""
echo "2. Cache update (if not already done):"
echo -e "   ${CYAN}gist-cache-rs update${NC}"
echo ""
echo "3. Search and run Gist:"
echo -e "   ${CYAN}gist-cache-rs run --preview keyword${NC}"
echo -e "   ${CYAN}gist-cache-rs run keyword bash${NC}"
echo ""
echo "Refer to the following documentation for details:"
echo "  • README.md - Feature details"
echo "  • docs/QUICKSTART.md - Quickstart Guide"
echo "  • docs/EXAMPLES.md - Examples Collection"
echo ""
print_info "If you encounter any issues, please check docs/INSTALL.md"
echo ""

# Cleanup temporary directory if created
if [ "${CLEANUP_TEMP:-false}" = true ] && [ -n "$TEMP_DIR" ]; then
    print_info "Cleaning up temporary directory..."
    cd "$HOME" || exit 1
    rm -rf "$TEMP_DIR"
    print_success "Cleanup complete"
    echo ""
fi

exit 0
