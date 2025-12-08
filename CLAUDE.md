# CLAUDE.md

This file provides guidance for Claude Code (claude.ai/code) when working on this repository.

## Quick Reference

For comprehensive documentation, see the [Architecture & Design](https://7rikazhexde.github.io/gist-cache-rs/developer-guide/architecture.html) page in our documentation.

## Project Overview

**gist-cache-rs** is a Rust CLI tool for efficiently caching, searching, and executing GitHub Gists. It offers fast incremental updates, multi-language script execution support, and content caching capabilities.

**Supported Platforms**: Linux, macOS, Windows 10 or later

**Supported Interpreters**: bash, sh, zsh, python3, ruby, node, php, perl, pwsh (PowerShell Core), TypeScript (ts-node, deno, bun), uv

## Essential Commands

### Development

```bash
# Build and test
cargo build
cargo build --release
cargo test
cargo test -- --nocapture

# Code quality checks
just check        # Run all checks
just fmt          # Format code
just lint         # Run clippy
just ci-check     # CI checks
```

### Testing

```bash
# Run application
cargo run -- update
cargo run -- run <query> [interpreter] [args...]
cargo run -- cache list
cargo run -- self update --check

# Coverage
cargo tarpaulin --out Stdout
cargo tarpaulin --out Html --output-dir coverage
```

## Documentation

### For comprehensive information

- **[Architecture & Design](https://7rikazhexde.github.io/gist-cache-rs/developer-guide/architecture.html)** - Module structure, design patterns, implementation details
- **[Testing Guide](https://7rikazhexde.github.io/gist-cache-rs/developer-guide/testing.html)** - Testing strategy and execution
- **[Coverage Analysis](https://7rikazhexde.github.io/gist-cache-rs/developer-guide/coverage.html)** - Coverage reports and metrics
- **[Test Inventory](https://7rikazhexde.github.io/gist-cache-rs/developer-guide/test-inventory.html)** - Complete test catalog

### Quick Architecture Overview

```text
src/
├── cache/         # 2-layer caching (metadata + content)
├── github/        # GitHub CLI wrapper
├── execution/     # Multi-interpreter script runner
├── search/        # Flexible search with multiple modes
├── self_update/   # Application self-update
├── cli.rs         # CLI argument processing
├── config.rs      # Configuration management
└── error.rs       # Error types
```

## Key Design Patterns

1. **2-Layer Caching**: Metadata cache + on-demand content cache for 20x speedup
2. **Incremental Updates**: GitHub API's `since` parameter for efficient updates
3. **GitHub CLI Integration**: Uses `gh` command for authentication and API access
4. **Multi-Interpreter Support**: Abstracts different interpreters with special handling
5. **Platform-Specific Paths**: Conditional compilation for Unix/Windows compatibility

## Important Notes

- All timestamps use ISO 8601 format without sub-seconds
- Cache path overridable via `GIST_CACHE_DIR` environment variable (for testing)
- Tests use `MockGitHubClient` for isolation
- Current coverage: 68.95% (163 tests: 120 unit + 43 integration)

## Release Process

```bash
# Update version
vim Cargo.toml CHANGELOG.md
git add Cargo.toml CHANGELOG.md
git commit -m "🔖 Bump version to X.Y.Z"

# Create and push tag
git tag vX.Y.Z
git push origin main
git push origin vX.Y.Z
```

GitHub Actions automatically builds releases for:

- Linux (x86_64)
- macOS (x86_64, aarch64)
- Windows (x86_64)

## Dependencies

**Runtime**: tokio, serde, clap, chrono, anyhow, thiserror, dirs, colored, self_update
**Development**: mockall, tempfile, assert_cmd

For detailed information, always refer to the [full documentation](https://7rikazhexde.github.io/gist-cache-rs/).
