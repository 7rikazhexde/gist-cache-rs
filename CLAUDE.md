# CLAUDE.md

This file provides guidance for Claude Code (claude.ai/code) when working on this repository.

## Project Overview

**gist-cache-rs** is a Rust CLI tool for efficiently caching, searching, and executing GitHub Gists. It offers fast incremental updates, multi-language script execution support, and content caching capabilities.

**Supported Platforms**: Linux, macOS, Windows 10 or later

<!-- markdownlint-disable-next-line MD013 -->
**Supported Interpreters**: bash, sh, zsh, python3, ruby, node, php, perl, pwsh (PowerShell Core), TypeScript (ts-node, deno, bun), uv

## Development Commands

### Build and Test

```bash
# Development build
cargo build

# Release build (optimized)
cargo build --release

# Local installation
cargo install --path .

# Run tests
cargo test

# Run tests with verbose output
cargo test -- --nocapture
```

### Code Quality Checks (via justfile)

```bash
# Run all checks (format, lint, test)
just check

# Format check only
just fmt-check

# Lint with clippy
just lint

# Run tests silently
just test

# Auto-format code
just fmt

# CI checks (treat warnings as errors)
just ci-check
```

### Application Execution

```bash
# Cache update
cargo run -- update
cargo run -- update --force
cargo run -- update --verbose

# Gist execution
cargo run -- run <query> [interpreter] [args...]
cargo run -- run --preview <query>
cargo run -- run --interactive <query>
cargo run -- run --force <query>  # Update cache before execution
cargo run -- run --download <query>  # Save to download folder

# Cache management
cargo run -- cache list
cargo run -- cache size
cargo run -- cache clear

# Application self-update
cargo run -- self update --check         # Check for updates only
cargo run -- self update                 # Update to latest version (GitHub Releases)
cargo run -- self update --from-source   # Update by building from source
cargo run -- self update --verbose
```

## Architecture Overview

### File Structure

```bash
src/
├── cache/              # Cache management layer
│   ├── content.rs      # Content cache (541 lines)
│   ├── types.rs        # Data type definitions (246 lines)
│   ├── update.rs       # Incremental update logic (849 lines)
│   └── mod.rs
├── github/             # GitHub API integration
│   ├── api.rs          # GitHub CLI wrapper (212 lines)
│   ├── client.rs       # Trait definitions (104 lines)
│   └── mod.rs
├── execution/          # Script execution
│   ├── runner.rs       # Multi-interpreter execution (758 lines)
│   └── mod.rs
├── search/             # Search functionality
│   ├── query.rs        # Search query processing (420 lines)
│   └── mod.rs
├── self_update/        # Self-update feature
│   ├── updater.rs      # Application update logic
│   └── mod.rs
├── cli.rs              # CLI argument processing (967 lines)
├── config.rs           # Configuration management (163 lines)
├── error.rs            # Error type definitions (160 lines)
├── lib.rs              # Library root
└── main.rs             # Entry point

Total: 18 files, approx. 4,600 lines
```

### Module Structure

The codebase follows a modular architecture with clear separation of concerns:

**`cache/`** - Cache management layer (2-layer caching structure)

- `types.rs`: Core data structures (`GistCache`, `GistInfo`, `GistFile`, `CacheMetadata`)
- `update.rs`: `CacheUpdater` handles incremental metadata cache updates using GitHub API`s `since` parameter. Automatically deletes corresponding content cache when Gist updates are detected.
- `content.rs`: `ContentCache` manages individual Gist content files in `~/.cache/gist-cache/contents/{gist_id}/{filename}`. Created on first execution, speeding up subsequent executions (approx. 20x).

**`github/`** - GitHub API integration

- `api.rs`: `GitHubApi` wraps GitHub CLI (`gh`) for authentication, rate limit checks, and gist retrieval.
- All GitHub operations use `gh` CLI instead of direct REST API calls.

**`search/`** - Search functionality

- `query.rs`: Implements `SearchQuery` with multiple modes (Auto, ID, Filename, Description).
- Interactive selection UI using numbered prompts.

**`execution/`** - Script execution

- `runner.rs`: `ScriptRunner` handles multi-interpreter execution (bash, python, ruby, node, php, perl, pwsh, TypeScript, uv).
- Supports both stdin-based and file-based execution modes.
- `uv` interpreter uses file-based execution for PEP 723 metadata support.
- `pwsh` (PowerShell Core) and `powershell` (Windows PowerShell) use file-based execution for compatibility with script execution policies.
- TypeScript interpreters (`ts-node`, `deno`, `bun`) use file-based execution for module resolution.
    - `ts-node`: Executes TypeScript on Node.js
    - `deno`: Uses `deno run` command in Deno runtime
    - `bun`: Executes in Bun runtime
- Interactive mode for scripts using `read`, etc.

**`self_update/`** - Application self-update feature

- `updater.rs`: `Updater` handles automatic updates from GitHub Releases or source.
- **GitHub Releases update**: Binary download using `self_update` crate.
- **Source build update**: Build update via git pull + cargo install.
- Supports update checks (`--check`), forced updates (`--force`), and version-specific updates.
- Repository path detection: environment variable → cargo metadata → error.
- Automatically fetches from origin/main if no tracking info is available.

**`config.rs`** - Configuration management

- Manages cache paths (platform-specific):
  - Overridable by environment variable `GIST_CACHE_DIR` (for testing).
  - Unix: `~/.cache/gist-cache/cache.json` and `~/.cache/gist-cache/contents/`
  - Windows: `%LOCALAPPDATA%\gist-cache\cache.json` and `%LOCALAPPDATA%\gist-cache\contents\`
- Manages download path: Uses `dirs::download_dir()` to conform to OS standards.
- Isolation in test environment: Can be tested without affecting actual user cache by setting `GIST_CACHE_DIR`.

**`error.rs`** - Centralized error handling using `thiserror`.

### Key Design Patterns

1. **Incremental Updates**: Metadata cache updates use GitHub API`s `since` parameter to fetch only changed gists. Timestamp stored in `last_updated` of `cache.json`.

2. **2-Layer Caching (On-demand)**:
   - **Metadata Cache**: Stores gist metadata (id, description, files, updated_at) in `cache.json`. Updated with the `update` command.
   - **Content Cache**: Stores actual script body in `contents/{gist_id}/{filename}`. Created on-demand during execution and automatically deleted when Gist updates.
   - **Cache Freshness Management**: The `update` command compares `updated_at` of new and old metadata, and deletes the content cache directory (`contents/{gist_id}/`) for updated Gists.

3. **GitHub CLI Integration**: Uses `gh` command for authentication and API access instead of direct REST API calls.

4. **Multi-Interpreter Support**: The execution layer abstracts different interpreters and implements special handling:
   - Shell scripts (bash/sh/zsh): Direct execution.
   - `uv`: File-based using `uv run` command for PEP 723 support.
   - `php`: Forced file-based execution for reliable argument handling.
   - `pwsh`/`powershell`: Forced file-based execution for script execution policy compatibility.
   - TypeScript (`ts-node`, `deno`, `bun`): Forced file-based execution for module resolution and runtime requirements.
     - `ts-node`: Executes TypeScript on Node.js.
     - `deno`: Uses `deno run` command in Deno runtime.
     - `bun`: Executes in Bun runtime.
   - Others: Standard stdin-based execution.

5. **Search Modes**: Supports flexible searching:
   - `Auto`: Detects if query is a Gist ID (32-character hexadecimal), or searches filename/description.
   - `Id`: Direct ID search.
   - `Filename`: Searches filenames only.
   - `Description`: Searches descriptions only.

6. **`--force` Option**: When `--force` is specified with the `run` command, it automatically executes the `update` command (incremental update, not `update --force`) before execution to get the latest Gist information. If the Gist was updated, the latest version is automatically fetched.

7. **`--download` Option**: When `--download` is specified with the `run` command, the Gist file is saved to the download folder (`~/Downloads`).
   Convenient for saving files separately from executable script caches.
   Content cache is also automatically created during download, speeding up subsequent executions. Can be used with other options (`--preview`, `--force`, `--interactive`, etc.).

## Important Implementation Details

### Date and Time Handling

All timestamps use ISO 8601 format (`%Y-%m-%dT%H:%M:%SZ`) without sub-seconds to maintain compatibility with the original bash implementation. Custom serializer/deserializer is available in `cache/types.rs`.

### Cache File Format

Structure of `cache.json`:

```json
{
  "metadata": {
    "last_updated": "2024-01-01T12:00:00Z",
    "total_count": 42,
    "github_user": "username"
  },
  "gists": [...] 
}
```

### Execution Modes

- **Stdin Mode** (default): Pipes script content directly to the interpreter.
- **File Mode** (uv, php, interactive): Creates a temporary file for execution.
- **Interactive Mode** (`-i`): Uses `inherit()` for stdio to support `read` command in scripts.
- **Preview Mode** (`-p`/`--preview`): Displays only Description, Files, and Gist content without executing the script. Can be combined with search modes (Auto, ID, Filename, Description).

### Platform-Specific Implementations

**Windows Support**:

- **Permission Settings**: Uses conditional compilation (`#[cfg(unix)]`) to run `chmod` only on Unix environments. On Windows, file executability is determined by file extension, so permission settings are not required.
- **Path Settings**: Uses platform-specific cache directories in `src/config.rs`.
  - Unix: `~/.cache/gist-cache`
  - Windows: `%LOCALAPPDATA%\gist-cache` (uses `dirs::cache_dir()`)
  - Download directory uses `dirs::download_dir()` for all platforms.
- **Installation Script**: Provides a PowerShell version (`script/setup.ps1`).

**Cross-Platform Design**:

- Explicit branching with conditional compilation (`cfg` attributes).
- Prioritizes platform-independent code.
- Degrade prevention to avoid affecting existing Linux/macOS environments.

### Rate Limiting

The Updater checks the rate limit and warns if remaining requests are less than 50. Forced full updates via `update --force` can consume a significant amount of rate limits as it fetches all gists.

### Content Cache Behavior Flow

1. **First Execution**: Fetches content from GitHub API and creates a cache in `contents/{gist_id}/{filename}` after execution.
2. **Subsequent Executions**: Reads from cache for faster execution (no network access, approx. 20x faster).
3. **Gist Update**: The `update` command detects changes in `updated_at` of metadata and automatically deletes the corresponding content cache directory (`contents/{gist_id}/`).
4. **First Execution After Update**: Fetches the latest version from API and creates a new cache.

### `--force` Option Behavior

When `run --force` is specified:

1. Automatically executes the `update` command (incremental update, not `update --force`) before execution.
2. If the Gist was updated, the content cache is deleted.
3. Fetches and executes the latest version.
4. Creates a new cache.

This ensures that even when Gists are frequently updated during development, the latest version is always executed.

## Tests

Tests use `tokio::test` for asynchronous functions and are placed in modules and inline using `#[cfg(test)]`. Currently, `src/cache/content.rs` has minimal test coverage.

Development dependencies:

- `assert_cmd`: For CLI integration tests.
- `tempfile`: For temporary test fixtures.

## Cache Management Commands

Content cache management functions implemented with `cache` subcommand (main.rs:287-412):

- `cache list`: Displays a list of cached Gists (ID, description, filename, update time).
- `cache size`: Displays the total size of the cache directory.
- `cache clean`: Deletes orphaned caches (not yet implemented, planned for future).
- `cache clear`: Deletes all content caches (with confirmation prompt).

Methods provided by `ContentCache` struct (src/cache/content.rs):

- `list_cached_gists()`: Get a list of cached Gist IDs.
- `total_size()`: Calculate the total size of the cache directory.
- `clear_all()`: Delete all caches.
- `read()`, `write()`, `exists()`: Read/write individual caches.

## Release Process

### Automated Release Builds

When a tag is pushed, GitHub Actions automatically builds and releases platform-specific binaries.

```bash
# Update version
vim Cargo.toml CHANGELOG.md
git add Cargo.toml CHANGELOG.md
git commit -m "🔖 Bump version to 0.5.0"

# Create and push tag
git tag v0.5.0
git push origin main
git push origin v0.5.0
```

### Build Target Platforms

- Linux (x86_64): `gist-cache-rs-linux-x86_64.tar.gz`
- macOS (x86_64): `gist-cache-rs-macos-x86_64.tar.gz`
- macOS (Apple Silicon): `gist-cache-rs-macos-aarch64.tar.gz`
- Windows (x86_64): `gist-cache-rs-windows-x86_64.zip`

### Workflow

Defined in `.github/workflows/release.yml`:

1. `create-release`: Creates release page, generates release notes.
2. `build-release`: Parallel builds (4 platforms), uploads assets.

For details, refer to [docs/SELF-UPDATE.md](docs/SELF-UPDATE.md#release-process).

## Dependencies

Primary runtime dependencies:

- `tokio`: Asynchronous runtime.
- `reqwest`: HTTP client (unused, remnant from direct API implementation era).
- `serde`/`serde_json`: Serialization.
- `clap`: CLI argument parsing.
- `chrono`: Date and time handling.
- `anyhow`/`thiserror`: Error handling.
- `dirs`: Platform-specific directory detection.
- `colored`: Terminal output coloring.
- `self_update`: Automatic updates from GitHub Releases.

Development dependencies:

- `mockall`: Mocking library (for external dependency testing).
- `tempfile`: Temporary files/directories (for testing).
- `assert_cmd`: For CLI testing (for future integration tests).

## Tests and Coverage

**Current Status**: 68.95% coverage, 163 tests (unit: 120, integration: 43).

**Test Composition**:

- **Unit Tests**: `#[cfg(test)]` modules in `src/`, uses `MockGitHubClient`.
- **Integration Tests**: CLI behavior, interpreter execution (bash/python/node etc.), platform-specific tests.
- **E2E Tests**: 26 functional verification test design documents in `docs/tests/`.

**Coverage Measurement**:

```bash
cargo tarpaulin --out Stdout
cargo tarpaulin --out Html --output-dir coverage
```

<!-- markdownlint-disable-next-line MD013 -->
For details, refer to [TESTING.md](docs/testing/TESTING.md), [COVERAGE.md](docs/testing/COVERAGE.md), and [TEST_INVENTORY.md](docs/testing/TEST_INVENTORY.md).