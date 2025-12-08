# Architecture and Design

This guide provides a comprehensive overview of gist-cache-rs's architecture, design patterns, and implementation details.

## Project Overview

**gist-cache-rs** is a Rust CLI tool for efficiently caching, searching, and executing GitHub Gists. It offers fast incremental updates, multi-language script execution support, and content caching capabilities.

**Supported Platforms**: Linux, macOS, Windows 10 or later

**Supported Interpreters**: bash, sh, zsh, python3, ruby, node, php, perl, pwsh (PowerShell Core), TypeScript (ts-node, deno, bun), uv

## Architecture Overview

### Module Structure

The codebase follows a modular architecture with clear separation of concerns:

```text
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

### Cache Module (`cache/`)

The cache layer implements a sophisticated 2-layer caching structure:

**`types.rs`**: Core data structures

- `GistCache`: Main cache container
- `GistInfo`: Individual gist metadata
- `GistFile`: File information
- `CacheMetadata`: Cache metadata and timestamps

**`update.rs`**: `CacheUpdater` implementation

- Handles incremental metadata cache updates using GitHub API's `since` parameter
- Automatically deletes corresponding content cache when Gist updates are detected
- Implements rate limit checking and warning system

**`content.rs`**: `ContentCache` implementation

- Manages individual Gist content files in `~/.cache/gist-cache/contents/{gist_id}/{filename}`
- Created on-demand during first execution
- Provides approximately 20x performance improvement for subsequent executions

### GitHub Module (`github/`)

Handles all GitHub API interactions:

**`api.rs`**: `GitHubApi` wrapper

- Wraps GitHub CLI (`gh`) for authentication
- Implements rate limit checks
- Handles gist retrieval operations
- All GitHub operations use `gh` CLI instead of direct REST API calls

**`client.rs`**: Trait definitions

- Defines the `GitHubClient` trait for dependency injection
- Enables testing with mock implementations

### Search Module (`search/`)

Implements flexible search functionality:

**`query.rs`**: `SearchQuery` implementation

- Multiple search modes:
  - **Auto**: Detects if query is a Gist ID (32-character hexadecimal), or searches filename/description
  - **ID**: Direct ID search
  - **Filename**: Searches filenames only
  - **Description**: Searches descriptions only
- Interactive selection UI using numbered prompts

### Execution Module (`execution/`)

Handles multi-interpreter script execution:

**`runner.rs`**: `ScriptRunner` implementation

- Supports multiple interpreters (bash, python, ruby, node, php, perl, pwsh, TypeScript, uv)
- Two execution modes:
  - **Stdin-based**: Default for most interpreters
  - **File-based**: Required for uv, php, pwsh, TypeScript interpreters

**Special Interpreter Handling**:

- **uv**: File-based execution for PEP 723 metadata support
- **php**: Forced file-based execution for reliable argument handling
- **pwsh/powershell**: File-based execution for script execution policy compatibility
- **TypeScript interpreters** (ts-node, deno, bun): File-based execution for module resolution
  - `ts-node`: Executes TypeScript on Node.js
  - `deno`: Uses `deno run` command in Deno runtime
  - `bun`: Executes in Bun runtime
- **Interactive mode**: Uses `inherit()` for stdio to support commands like `read`

### Self-Update Module (`self_update/`)

Implements application self-update functionality:

**`updater.rs`**: `Updater` implementation

- **GitHub Releases update**: Binary download using `self_update` crate
- **Source build update**: Build update via git pull + cargo install
- Supports update checks (`--check`), forced updates (`--force`), and version-specific updates
- Repository path detection: environment variable → cargo metadata → error
- Automatically fetches from origin/main if no tracking info is available

### Configuration (`config.rs`)

Manages application configuration:

**Cache paths** (platform-specific):

- Overridable by environment variable `GIST_CACHE_DIR` (for testing)
- Unix: `~/.cache/gist-cache/cache.json` and `~/.cache/gist-cache/contents/`
- Windows: `%LOCALAPPDATA%\gist-cache\cache.json` and `%LOCALAPPDATA%\gist-cache\contents\`

**Download path**: Uses `dirs::download_dir()` to conform to OS standards

**Test isolation**: Can be tested without affecting actual user cache by setting `GIST_CACHE_DIR`

### Error Handling (`error.rs`)

Centralized error handling using `thiserror` crate for type-safe error management.

## Key Design Patterns

### 1. Incremental Updates

Metadata cache updates use GitHub API's `since` parameter to fetch only changed gists. Timestamp stored in `last_updated` field of `cache.json`.

### 2. 2-Layer Caching (On-demand)

**Metadata Cache**:

- Stores gist metadata (id, description, files, updated_at) in `cache.json`
- Updated with the `update` command

**Content Cache**:

- Stores actual script body in `contents/{gist_id}/{filename}`
- Created on-demand during execution
- Automatically deleted when Gist updates

**Cache Freshness Management**:
The `update` command compares `updated_at` of new and old metadata, and deletes the content cache directory (`contents/{gist_id}/`) for updated Gists.

### 3. GitHub CLI Integration

Uses `gh` command for authentication and API access instead of direct REST API calls. This provides:

- Automatic authentication handling
- Consistent credential management
- Better error messages

### 4. Multi-Interpreter Support

The execution layer abstracts different interpreters and implements special handling:

- **Shell scripts** (bash/sh/zsh): Direct execution
- **uv**: File-based using `uv run` command for PEP 723 support
- **php**: Forced file-based execution for reliable argument handling
- **pwsh/powershell**: Forced file-based execution for script execution policy compatibility
- **TypeScript** (ts-node, deno, bun): Forced file-based execution for module resolution
- **Others**: Standard stdin-based execution

### 5. Search Modes

Supports flexible searching:

- **Auto**: Intelligent detection of query type
- **ID**: Direct ID search for exact matches
- **Filename**: Searches filenames only
- **Description**: Searches descriptions only

### 6. `--force` Option

When `--force` is specified with the `run` command:

1. Automatically executes the `update` command (incremental update, not `update --force`)
2. If the Gist was updated, the content cache is deleted
3. Fetches and executes the latest version
4. Creates a new cache

This ensures that the latest version is always executed during active development.

### 7. `--download` Option

When `--download` is specified with the `run` command:

- Saves the Gist file to the download folder (`~/Downloads`)
- Convenient for saving files separately from executable script caches
- Content cache is also automatically created during download
- Can be used with other options (`--preview`, `--force`, `--interactive`, etc.)

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

- **Stdin Mode** (default): Pipes script content directly to the interpreter
- **File Mode** (uv, php, interactive): Creates a temporary file for execution
- **Interactive Mode** (`-i`): Uses `inherit()` for stdio to support `read` command in scripts
- **Preview Mode** (`-p`/`--preview`): Displays Description, Files, and Gist content without execution

### Platform-Specific Implementations

**Windows Support**:

- **Permission Settings**: Uses conditional compilation (`#[cfg(unix)]`) to run `chmod` only on Unix
- **Path Settings**: Uses platform-specific cache directories
  - Unix: `~/.cache/gist-cache`
  - Windows: `%LOCALAPPDATA%\gist-cache`
- **Installation Script**: Provides PowerShell version (`script/setup.ps1`)

**Cross-Platform Design**:

- Explicit branching with conditional compilation (`cfg` attributes)
- Prioritizes platform-independent code
- Prevents degradation in existing Linux/macOS environments

### Rate Limiting

The Updater checks the rate limit and warns if remaining requests are less than 50. Forced full updates via `update --force` can consume significant rate limits as it fetches all gists.

### Content Cache Behavior Flow

1. **First Execution**: Fetches content from GitHub API and creates a cache after execution
2. **Subsequent Executions**: Reads from cache for faster execution (no network access, ~20x faster)
3. **Gist Update**: The `update` command detects changes and automatically deletes the content cache
4. **First Execution After Update**: Fetches the latest version from API and creates a new cache

### `--force` Option Behavior

When `run --force` is specified:

1. Automatically executes the `update` command (incremental update)
2. If the Gist was updated, the content cache is deleted
3. Fetches and executes the latest version
4. Creates a new cache

This ensures that even when Gists are frequently updated during development, the latest version is always executed.

## Cache Management

Content cache management functions implemented with `cache` subcommand:

- `cache list`: Displays a list of cached Gists (ID, description, filename, update time)
- `cache size`: Displays the total size of the cache directory
- `cache clean`: Deletes orphaned caches (not yet implemented, planned for future)
- `cache clear`: Deletes all content caches (with confirmation prompt)

Methods provided by `ContentCache` struct:

- `list_cached_gists()`: Get a list of cached Gist IDs
- `total_size()`: Calculate the total size of the cache directory
- `clear_all()`: Delete all caches
- `read()`, `write()`, `exists()`: Read/write individual caches

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
cargo run -- self update                 # Update to latest version
cargo run -- self update --from-source   # Update by building from source
cargo run -- self update --verbose
```

## Dependencies

### Primary Runtime Dependencies

- `tokio`: Asynchronous runtime
- `reqwest`: HTTP client (unused, remnant from direct API implementation era)
- `serde`/`serde_json`: Serialization
- `clap`: CLI argument parsing
- `chrono`: Date and time handling
- `anyhow`/`thiserror`: Error handling
- `dirs`: Platform-specific directory detection
- `colored`: Terminal output coloring
- `self_update`: Automatic updates from GitHub Releases

### Development Dependencies

- `mockall`: Mocking library (for external dependency testing)
- `tempfile`: Temporary files/directories (for testing)
- `assert_cmd`: For CLI testing (for future integration tests)

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

1. `create-release`: Creates release page, generates release notes
2. `build-release`: Parallel builds (4 platforms), uploads assets

For details, refer to the [Self-Update Guide](self-update.md).
