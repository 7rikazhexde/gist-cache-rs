# CLAUDE.md

This file provides guidance for Claude Code (claude.ai/code) when working on this repository.

## Quick Reference

For comprehensive documentation, see the [Architecture & Design](https://7rikazhexde.github.io/gist-cache-rs/developer-guide/architecture.html) page in our documentation.

## Project Overview

**gist-cache-rs** is a Rust CLI tool for efficiently caching, searching, and executing GitHub Gists. It offers fast incremental updates, multi-language script execution support, and content caching capabilities.

**Current Version**: 0.8.6
**Rust Edition**: 2024
**Minimum Rust Version**: 1.85
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
cargo run -- config show
cargo run -- completions bash

# Coverage
cargo tarpaulin --out Stdout
cargo tarpaulin --out Html --output-dir coverage
```

## New Features in v0.8.6

### Advanced Interpreter Configuration

The tool now supports extension-based interpreter mapping with priority-based resolution:

```bash
# Set per-extension interpreters
gist-cache-rs config set defaults.interpreter.py python3
gist-cache-rs config set defaults.interpreter.rb ruby
gist-cache-rs config set defaults.interpreter.ts deno
gist-cache-rs config set defaults.interpreter."*" bash  # Wildcard fallback

# Legacy single interpreter still supported
gist-cache-rs config set defaults.interpreter bash
```

**Interpreter Resolution Priority**:

1. Command-line argument (explicit override)
2. Shebang line (e.g., `#!/usr/bin/env python3`)
3. User configuration (extension-based mapping)
4. Filename heuristics (e.g., `Makefile` → `make`)
5. Content analysis (language detection)
6. Wildcard fallback (`"*"`) or `bash`

### Shell Completions

Generate completion scripts for your shell:

```bash
# Generate and install completions
gist-cache-rs completions bash > ~/.local/share/bash-completion/completions/gist-cache-rs
gist-cache-rs completions zsh > ~/.zsh/completions/_gist-cache-rs
gist-cache-rs completions fish > ~/.config/fish/completions/gist-cache-rs.fish
gist-cache-rs completions powershell > gist-cache-rs.ps1
```

Supported shells: Bash, Zsh, Fish, PowerShell

### Configuration Management

Full configuration management with CLI commands:

```bash
# View current configuration
gist-cache-rs config show

# Get specific value
gist-cache-rs config get defaults.interpreter.py

# Set configuration values
gist-cache-rs config set execution.confirm_before_run true
gist-cache-rs config set cache.retention_days 30

# Edit config file in $EDITOR
gist-cache-rs config edit

# Reset to defaults
gist-cache-rs config reset
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
├── cli.rs         # CLI argument processing
├── config.rs      # Configuration management
└── error.rs       # Error types
```

## Key Design Patterns

1. **2-Layer Caching**: Metadata cache + on-demand content cache for 20x speedup
2. **Incremental Updates**: GitHub API's `since` parameter for efficient updates
3. **GitHub CLI Integration**: Uses `gh` command for authentication and API access
4. **Priority-Based Interpreter Resolution** (v0.8.6+): Multi-level detection system
   - Command-line argument → Shebang → User config → Filename heuristics → Content analysis → Fallback
5. **Flexible Configuration**: Extension-based interpreter mapping with wildcard fallback support
6. **Shell Completions**: Auto-generated completions for Bash, Zsh, Fish, PowerShell
7. **Interactive UI**: Progress bars, spinners, and arrow-key navigation using `indicatif` and `dialoguer`
8. **Platform-Specific Paths**: Conditional compilation for Unix/Windows compatibility

## Important Notes

- All timestamps use ISO 8601 format without sub-seconds
- Cache path overridable via `GIST_CACHE_DIR` environment variable (for testing)
- Config path also follows `GIST_CACHE_DIR` override for testing isolation
- Tests use `MockGitHubClient` for isolation
- Current test count: 148 tests
- Configuration stored in platform-specific locations:
  - Unix: `~/.config/gist-cache/config.toml`
  - Windows: `%APPDATA%\gist-cache\config.toml`

## Release Process

**CRITICAL: ALWAYS update Cargo.toml version BEFORE creating tags. This is MANDATORY and non-negotiable.**

```bash
# Step 1: Update version in Cargo.toml (MANDATORY - DO NOT SKIP)
vim Cargo.toml  # Change version = "X.Y.Z"

# Step 2: Update CHANGELOG.md (if exists)
vim CHANGELOG.md

# Step 3: Commit version bump
git add Cargo.toml CHANGELOG.md
git commit -m "🔖 Bump version to X.Y.Z"

# Step 4: Push to main
git push origin main

# Step 5: Create and push tag (ONLY after Cargo.toml is updated)
git tag -a vX.Y.Z -m "Release vX.Y.Z"
git push origin vX.Y.Z
```

**Version Update Checklist:**

- [ ] Update `version` in `Cargo.toml`
- [ ] Update `CHANGELOG.md` (if applicable)
- [ ] Commit changes with "🔖 Bump version to X.Y.Z"
- [ ] Push to main branch
- [ ] Create annotated tag with `-a` flag
- [ ] Push tag to trigger release workflow

GitHub Actions automatically builds releases for:

- Linux (x86_64)
- macOS (x86_64, aarch64)
- Windows (x86_64)

## Dependencies

**Runtime**:

- Core: tokio, serde, anyhow, thiserror
- CLI: clap (v4.5), clap_complete (v4.5), colored (v3.0)
- UI: dialoguer (v0.12), indicatif (v0.18)
- Utils: chrono (v0.4), dirs (v6.0), toml (v0.9), tokei (v13.0)

**Development**:

- Testing: mockall (v0.14), assert_cmd (v2.0), predicates (v3.1), serial_test (v3.0)
- Utils: tempfile (v3.8)

For detailed information, always refer to the [full documentation](https://7rikazhexde.github.io/gist-cache-rs/).
