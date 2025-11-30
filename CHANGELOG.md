# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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

[Unreleased]: https://github.com/7rikazhexde/gist-cache-rs/compare/v0.5.0...HEAD
[0.5.0]: https://github.com/7rikazhexde/gist-cache-rs/compare/v0.4.0...v0.5.0
[0.4.0]: https://github.com/7rikazhexde/gist-cache-rs/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/7rikazhexde/gist-cache-rs/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/7rikazhexde/gist-cache-rs/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/7rikazhexde/gist-cache-rs/releases/tag/v0.1.0
