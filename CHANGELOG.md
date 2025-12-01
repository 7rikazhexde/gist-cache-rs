# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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

## [0.5.3] - 2025-12-01

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

## [0.4.0] - 2025-01-30

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

## [0.3.0] - 2024-XX-XX

### Added
- Interactive execution mode (`-i` flag)
- Preview mode (`-p` flag)
- Force update before run (`--force` flag)
- Multiple search modes (Auto, ID, Filename, Description)
- uv interpreter support with PEP 723 metadata

### Changed
- Improved search query handling
- Enhanced error messages

## [0.2.0] - 2024-XX-XX

### Added
- Two-tier caching system (metadata + content)
- Differential update using GitHub API `since` parameter
- Multiple interpreter support (bash, python, ruby, node, php, perl)
- Content cache with automatic cleanup on Gist updates

### Changed
- Migrated from direct API calls to GitHub CLI (`gh`)
- Optimized cache update performance

## [0.1.0] - 2024-XX-XX

### Added
- Initial release
- Basic Gist caching functionality
- Search and execute Gists
- Metadata caching

[Unreleased]: https://github.com/7rikazhexde/gist-cache-rs/compare/v0.5.5...HEAD
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
