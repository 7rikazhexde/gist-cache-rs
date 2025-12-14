# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.8.5] - 2025-12-14

### Added

- **Install Script Enhancements (Shell Completion)**
  - Added functionality to `script/setup.ps1` and `script/setup.sh` to optionally configure shell completions.
  - Detects user's shell (PowerShell, Bash, Zsh, Fish), checks for existing config, and performs setup or update.

### Changed

- **CLI (Config Show Output)**
  - Improved output for `gist-cache-rs config show` when no configuration settings are present.
  - Displays a helpful message guiding users on available options and how to set them.
  - Added integration tests for this new behavior.

- **Documentation**
  - Updated `.claude/docs/feature_update_plan.md` to reflect the latest progress in the implementation roadmap.
  - Updated `.claude/docs/feature_update_plan.md` to reflect the `config show` improvement.

## [0.8.4] - 2025-12-14

### Changed

- **CLI (Tab Completion)**
  - Refactored `clap::Subcommand` enum for `ConfigCommands` to use dedicated `Args` structs instead of inline field definitions.
  - Enabled tab completion for `gist-cache-rs config` subcommands (set, get, show, edit, reset). (Closes #36)

- **Documentation**
  - Improved `book/src/user-guide/shell-completions.md` to clarify setup and activation steps for PowerShell, Bash, and Fish.

## [0.8.3] - 2025-12-14

### Added

- **Configuration File Support** (Closes #34)
  - User configuration system with both CLI and file-based interfaces
  - CLI commands: `config set`, `config get`, `config show`, `config edit`, `config reset`
  - Configuration file locations:
    - Linux/macOS: `~/.config/gist-cache/config.toml`
    - Windows: `%APPDATA%\gist-cache\config.toml`
  - Configuration options:
    - `defaults.interpreter`: Set default interpreter for script execution (bash, python3, ruby, node, etc.)
    - `execution.confirm_before_run`: Enable/disable confirmation prompt before execution (for safety)
    - `cache.retention_days`: Set cache retention period in days
  - Support for `GIST_CACHE_DIR` environment variable for test isolation
  - Comprehensive documentation with practical examples and use cases

### Changed

- **Dependencies**
  - Added `toml = "0.9"` for TOML configuration file parsing

- **Configuration Module**
  - Extended `Config` struct with `config_file` and `user_config` fields
  - Added `UserConfig`, `DefaultsConfig`, `ExecutionConfig`, `CacheConfig` structs
  - Implemented config file operations: load, save, set, get, reset
  - Enhanced `Config::new()` to check `GIST_CACHE_DIR` for both config and cache directories

- **Documentation**
  - Created comprehensive Configuration Guide (book/src/user-guide/configuration.md)
  - Updated README.md with configuration feature and examples
  - Updated Quick Start Guide with configuration section
  - Updated Usage Examples with configuration examples
  - Fixed all internal markdown links in user-guide documentation

### Tests

- Added 3 integration tests for config functionality (`test_config_set_get`, `test_config_show`, `test_config_reset`)
- All 173 tests passing (136 unit + 30 integration + 7 execution)
- No feature degradation

### Benefits

- **User Convenience**: Set default interpreter once, use everywhere
- **Safety**: Optional execution confirmation for cautious users
- **Maintenance**: Configurable cache retention for disk space management
- **Flexibility**: Both CLI and file-based configuration for different workflows
- **Cross-platform**: Platform-aware config file locations

## [0.8.2] - 2025-12-14

### Added

- **JSON Output Format for Cache List** (Closes #32)
  - New `--format` flag for `cache list` command with options: `text` (default) and `json`
  - Machine-readable JSON output for scripting and automation use cases
  - JSON output includes: id, description, files array, and updated_at timestamp
  - Example usage with jq for filtering and data extraction
  - Perfect for integration with other tools and workflow automation

### Changed

- **CLI**
  - Added `OutputFormat` enum (`Text`, `Json`) for output format selection
  - Modified `CacheCommands::List` to accept `ListArgs` with format parameter
  - Implemented `GistListItem` struct for JSON serialization
  - Maintained backward compatibility (text format is default)

- **Configuration**
  - Consolidated markdownlint configuration to `.markdownlint.jsonc`
  - Removed redundant `.markdownlintrc` file
  - Added MD040 rule configuration (fenced code blocks language specification)

- **Documentation**
  - Updated README.md: Added JSON output format feature and usage examples with jq
  - Updated book/src/user-guide/examples.md: Comprehensive JSON usage examples including filtering, extraction, and counting

### Tests

- Added 2 integration tests for JSON format (`test_cache_list_json_format_empty`, `test_cache_list_json_format`)
- All 170 tests passing (133 unit + 27 integration + 10 execution)
- No feature degradation

## [0.8.1] - 2025-12-13

### Added

- **Progress Display** (Closes #28)
  - Visual feedback during long-running operations
  - Spinner: Displayed while fetching Gist information from GitHub API
  - Progress bar: Shown when processing 10+ Gists with percentage and count
  - Integration with `--verbose` flag (verbose: detailed logs, normal: progress indicators)
  - Improved user experience with visual feedback for cache update operations

### Changed

- **Dependencies**
  - Added `indicatif = "0.17"` for progress visualization

- **Documentation**
  - Updated README.md: Added progress display to features list and usage examples
  - Updated book/src/user-guide/quickstart.md: Added progress display examples with normal/verbose mode outputs

### Tests

- Added 2 integration tests for progress display functionality
- All 163 tests passing (138 unit + 25 integration)
- No feature degradation

## [0.8.0] - 2025-12-13

### Added

- **Shell Completion Support** (Closes #26)
  - New `completions` subcommand to generate shell-specific completion scripts
  - Support for 4 shells: Bash, Zsh, Fish, and PowerShell
  - Tab completion for commands, subcommands, and options
  - Comprehensive documentation with installation guides for all supported shells
  - Backup and restore procedures for shell configuration files
  - Real-world usage examples for WSL2/Linux/macOS and Windows
  - Platform-specific setup instructions

### Changed

- **Cross-platform Compatibility**
  - Improved interpreter validation using platform-specific commands (`where` on Windows, `which` on Unix)
  - Enhanced Windows compatibility for custom interpreter detection

- **Documentation**
  - Added `book/src/user-guide/shell-completions.md` with comprehensive setup guide
  - Updated README.md with shell completion quick start instructions
  - Added configuration file examples and actual usage demonstrations

### Removed

- **Dependency Cleanup**
  - Removed unused `reqwest` dependency (~500KB binary size reduction)
  - Cleaned up associated error types

### Tests

- Added 5 unit tests for shell completion generation
- Added 8 integration tests for CLI completions
- All 161 tests passing (138 unit + 23 integration)
- No feature degradation

## [0.7.0] - 2025-12-11

### Added

- **Cache Clean Command** (Closes #24)
  - New `cache clean` subcommand to remove old and orphaned cache entries
  - `--older-than <DAYS>` flag to remove entries older than specified days
  - `--orphaned` flag to remove orphaned content cache files
  - `--dry-run` flag to preview deletions without actually removing files
  - Detailed output showing files removed and space reclaimed

### Changed

- **Cache Management**
  - Enhanced cache management with selective cleanup capabilities
  - Improved cache maintenance for better storage efficiency

### Tests

- Added 10 comprehensive tests for cache clean functionality
- All 154 tests passing
- No feature degradation

### Documentation

- Updated README.md with cache clean examples
- Updated book/src/user-guide/quickstart.md
- Updated book/src/user-guide/examples.md with detailed cache clean examples
- Updated book/src/developer-guide/architecture.md

## [0.6.3] - 2025-12-09

### Fixed

- **Release Workflow**
  - Simplified release workflow logic to resolve linter warnings
  - Unified archive naming to include 'v' prefix (e.g., `gist-cache-rs-v0.6.3-...`)

### Changed

- **Documentation**
  - Updated installation instructions in release notes to reflect new archive names
- **Repository**
  - Added `.claude/settings.local.json` to `.gitignore` to exclude user-specific settings

## [0.6.2] - 2025-12-09

### Removed

- **Self-Update Feature** (Closes #22)
  - Removed `gist-cache-rs self update` command to unify installation methods
  - Removed `src/self_update/` module (~450 lines)
  - Removed `self-replace` and `self_update` dependencies
  - Users should now update using `cargo install gist-cache-rs`
  - This change eliminates redundancy and aligns with standard Rust tooling

### Changed

- **Documentation**
  - Simplified documentation badge display name from "Deploy Docs" to "Docs"
  - Updated all documentation to reflect removal of self-update feature
  - Updated release workflow to guide users to use `cargo install` for updates

### Fixed

- Fixed test assertions to use English messages consistently

## [0.6.1] - 2025-12-09

### Changed

- **Documentation Restructuring**
  - Reorganized mdbook structure with clear section separators
  - Improved introduction page with better navigation and feature highlights
  - Added comprehensive Architecture & Design guide in developer section
  - Simplified README.md to focus on quick start and documentation links
  - Updated CLAUDE.md to reference online documentation
  - Removed duplicate docs/ directory - all content now in mdbook (book/src/)
  - Fixed all internal documentation links
  - Changed documentation badge to reflect deployment status

### Added

- **Repository Management**
  - Branch protection settings for main branch (PR-based workflow)
  - Markdownlint configuration for consistent documentation style

## [0.6.0] - 2025-12-08

### Added

- **mdbook Documentation Structure**
  - Comprehensive documentation organized into three sections:
    - User Guide: Installation, Quick Start, Examples, Self-Update
    - Developer Guide: Testing, Coverage, Test Inventory, GitHub CLI Testing
    - Test Specifications: Detailed test results and test sets
  - GitHub Pages deployment workflow for automatic documentation publishing
  - Setup guide for mdbook configuration and deployment (docs/MDBOOK_SETUP.md)

- **Development Tools**
  - Pre-commit hooks configuration using prek/pre-commit
  - markdownlint configuration for consistent markdown style

### Changed

- **Internationalization (i18n)**
  - Translated all source code comments and error messages from Japanese to English
  - Translated all documentation files to English:
    - README.md, CLAUDE.md
    - docs/: INSTALL.md, QUICKSTART.md, EXAMPLES.md, SELF-UPDATE.md
    - docs/testing/: All testing documentation
    - docs/tests/: All test specification documents
  - Translated setup scripts (setup.sh, setup.ps1) and user-facing messages to English

## [0.5.8] - 2025-12-04

### Changed

- Crate publishing settings

## [0.5.7] - 2025-12-04

### Fixed

- Self-update archive path detection (Windows and Linux)
  - Fixed incorrect `bin_path_in_archive` template causing path mismatch
  - Changed directory path from `{{ bin }}-{{ version }}-{{ target }}` to `gist-cache-rs-{{ version }}-{{ target }}`
  - Added platform-specific `bin_name` configuration (`.exe` extension on Windows)
  - Now correctly matches actual archive structure on all platforms
  - Resolves "Could not find the required path in the archive" error

## [0.5.6] - 2025-12-02

### Added

- Pre-commit hooks for shell scripts and GitHub Actions workflows
  - Added shellcheck for shell script linting
  - Added actionlint for GitHub Actions workflow linting
  - Improved code quality checks in development workflow

### Changed

- Documentation improvements
  - Reduced documentation redundancy (~296 lines)
  - Simplified installation instructions in README
  - Consolidated cache information into single section
  - Simplified self-update documentation
  - Simplified test documentation in CLAUDE.md
  - Removed temporary work log file

### Fixed

- GitHub Actions workflow improvements
  - Fixed shellcheck warnings in release.yml
  - Fixed shellcheck warnings in pre-commit-autoupdate.yml
  - Proper quoting of variables in workflow scripts

## [0.5.5] - 2025-12-02

### Fixed

- Self-update binary path detection
  - Added `bin_path_in_archive` configuration to locate binaries within versioned subdirectories
  - Resolves "Could not find the required path in the archive" error
  - Now correctly extracts binaries from archive structure

## [0.5.4] - 2025-12-02

### Fixed

- Self-update archive extraction
  - Fixed directory structure in release archives
  - Binaries now placed in versioned subdirectories (e.g., `gist-cache-rs-0.5.4-x86_64-pc-windows-msvc/`)
  - Matches self_update crate's expected archive format
  - Resolves "Could not find the required path in the archive" error

## [0.5.3] - 2025-12-02

### Fixed

- Self-update binary detection in archives
  - Updated release asset naming to include version numbers
  - Asset names now follow pattern: `gist-cache-rs-{VERSION}-{TARGET}.tar.gz`
  - Matches self_update crate's expected naming convention
  - Fixed "Could not find the required path in the archive" error on all platforms

### Changed

- Release workflow improvements
  - Simplified artifact management with environment variables
  - Updated installation instructions to use new asset naming
  - Unified archive format (tar.gz) across all platforms

## [0.5.2] - 2025-12-01

### Fixed

- Binary name detection in self-update for Windows
  - Explicitly set `.exe` extension for Windows builds
  - Fixed "Could not find the required path in the archive" error
  - Archive extraction now works correctly on all platforms

## [0.5.1] - 2025-12-01

### Fixed

- Self-update archive extraction errors on all platforms
  - Added `archive-tar` feature to `self_update` crate
  - Fixed tar.gz extraction failures on Linux/macOS
- Windows `--from-source` access denied error
  - Replaced `cargo install` with `cargo build` + `self-replace`
  - Running executable can now be safely replaced on Windows
- Windows archive format compatibility
  - Changed from `.zip` to `.tar.gz` for consistency
  - Fixed "Compression method not supported" error
  - All platforms now use unified tar.gz format

### Changed

- Improved justfile recipes
  - Added default recipe and better comments
  - Added `test-verbose` and `build-release` recipes
  - Stricter CI checks with RUSTFLAGS="-D warnings"

## [0.5.0] - 2025-11-30

### Added

- Self-update functionality with `gist-cache-rs self update` command
  - GitHub Releases-based binary updates
  - Source-based updates with `--from-source` option
  - Update checking with `--check` flag
  - Force updates with `--force` flag
  - Version-specific updates with `--version` flag
  - Verbose mode with `--verbose` flag
- Automated release builds for multiple platforms via GitHub Actions
  - Linux (x86_64)
  - macOS (x86_64 and Apple Silicon)
  - Windows (x86_64)

### Changed

- Repository path detection now supports multiple strategies
  - Environment variable (`GIST_CACHE_REPO`)
  - Cargo metadata (`workspace_root`)
  - Helpful error messages with instructions

### Fixed

- Git pull now gracefully handles missing tracking information
  - Automatic fallback to `origin/main`

## [0.4.0] - 2025-11-07

### Added

- Windows platform support
- PowerShell Core (pwsh) interpreter support
- TypeScript execution via ts-node, deno, and bun
- Content cache management commands (`cache list`, `cache size`, `cache clear`)
- Download mode (`--download` flag) to save Gists to Downloads folder
- Comprehensive test coverage (68.95% overall)

### Changed

- Platform-specific cache directories
  - Linux/macOS: `~/.cache/gist-cache/`
  - Windows: `%LOCALAPPDATA%\gist-cache\`
- File-based execution for specific interpreters (uv, php, pwsh, TypeScript)
- Improved error handling and user messages

### Fixed

- Platform-specific permission handling (Unix vs Windows)
- Path handling for cross-platform compatibility

## [0.3.0] - 2025-11-03

### Added

- Interactive execution mode (`-i` flag)
- Preview mode (`-p` flag)
- Force update before run (`--force` flag)
- Multiple search modes (Auto, ID, Filename, Description)
- uv interpreter support with PEP 723 metadata

### Changed

- Improved search query handling
- Enhanced error messages

## [0.2.0] - 2025-11-03

### Added

- Two-tier caching system (metadata + content)
- Differential update using GitHub API `since` parameter
- Multiple interpreter support (bash, python, ruby, node, php, perl)
- Content cache with automatic cleanup on Gist updates

### Changed

- Migrated from direct API calls to GitHub CLI (`gh`)
- Optimized cache update performance

## [0.1.0]

### Added

- Initial release
- Basic Gist caching functionality
- Search and execute Gists
- Metadata caching

[Unreleased]: https://github.com/7rikazhexde/gist-cache-rs/compare/v0.8.1...HEAD
[0.8.1]: https://github.com/7rikazhexde/gist-cache-rs/compare/v0.8.0...v0.8.1
[0.8.0]: https://github.com/7rikazhexde/gist-cache-rs/compare/v0.7.0...v0.8.0
[0.7.0]: https://github.com/7rikazhexde/gist-cache-rs/compare/v0.6.3...v0.7.0
[0.6.3]: https://github.com/7rikazhexde/gist-cache-rs/compare/v0.6.2...v0.6.3
[0.6.2]: https://github.com/7rikazhexde/gist-cache-rs/compare/v0.6.1...v0.6.2
[0.6.1]: https://github.com/7rikazhexde/gist-cache-rs/compare/v0.6.0...v0.6.1
[0.6.0]: https://github.com/7rikazhexde/gist-cache-rs/compare/v0.5.8...v0.6.0
[0.5.8]: https://github.com/7rikazhexde/gist-cache-rs/compare/v0.5.7...v0.5.8
[0.5.7]: https://github.com/7rikazhexde/gist-cache-rs/compare/v0.5.6...v0.5.7
[0.5.6]: https://github.com/7rikazhexde/gist-cache-rs/compare/v0.5.5...v0.5.6
[0.5.5]: https://github.com/7rikazhexde/gist-cache-rs/compare/v0.5.4...v0.5.5
[0.5.4]: https://github.com/7rikazhexde/gist-cache-rs/compare/v0.5.3...v0.5.4
[0.5.3]: https://github.com/7rikazhexde/gist-cache-rs/compare/v0.5.2...v0.5.3
[0.5.2]: https://github.com/7rikazhexde/gist-cache-rs/compare/v0.5.1...v0.5.2
[0.5.1]: https://github.com/7rikazhexde/gist-cache-rs/compare/v0.5.0...v0.5.1
[0.5.0]: https://github.com/7rikazhexde/gist-cache-rs/compare/v0.4.0...v0.5.0
[0.4.0]: https://github.com/7rikazhexde/gist-cache-rs/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/7rikazhexde/gist-cache-rs/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/7rikazhexde/gist-cache-rs/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/7rikazhexde/gist-cache-rs/releases/tag/v0.1.0
