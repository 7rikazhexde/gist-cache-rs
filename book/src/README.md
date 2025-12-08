# gist-cache-rs Documentation

Welcome to the official documentation for **gist-cache-rs**, a high-performance CLI tool written in Rust for efficiently caching, searching, and executing GitHub Gists.

## What is gist-cache-rs?

gist-cache-rs provides a streamlined workflow for managing and executing your GitHub Gists locally. It features a sophisticated 2-layer caching system that dramatically reduces execution time and network overhead, making it perfect for developers who frequently use Gists as script repositories.

## Key Features

- ⚡ **High Performance** - Lightning-fast caching and search operations implemented in Rust
- 🔄 **Smart Updates** - Incremental cache updates that only fetch what's changed
- 💾 **2-Layer Caching** - Intelligent caching of both metadata and content for 20x faster execution
- 🔍 **Flexible Search** - Multiple search modes: by ID, filename, or description
- ▶️ **Multi-Language Support** - Execute scripts in bash, python, ruby, node, php, perl, pwsh, TypeScript and more
- 💬 **Interactive Mode** - Full support for interactive scripts
- 📦 **Modern Python** - uv support with PEP 723 metadata compatibility
- 📥 **Easy Downloads** - Save Gist files directly to your download folder
- 🗂️ **Cache Management** - Powerful commands for cache inspection and maintenance
- 🔃 **Self-Updating** - Keep your tool up-to-date with built-in update functionality

## Platform Support

gist-cache-rs works seamlessly across Linux, macOS, and Windows 10 or later.

## Quick Navigation

### For Users

Get started quickly with these essential guides:

- **[Installation Guide](user-guide/installation.md)** - Set up gist-cache-rs on your system
- **[Quick Start](user-guide/quickstart.md)** - Get running in 5 minutes
- **[Usage Examples](user-guide/examples.md)** - Real-world usage patterns
- **[Self-Update Guide](user-guide/self-update.md)** - Keep your tool current

### For Developers

Contributing to the project or want to understand the internals?

- **[Architecture](developer-guide/architecture.md)** - Project structure and design patterns
- **[Testing Guide](developer-guide/testing.md)** - Testing strategy and execution
- **[Coverage Reports](developer-guide/coverage.md)** - Test coverage analysis
- **[Test Inventory](developer-guide/test-inventory.md)** - Complete test catalog

### Test Specifications

Detailed functional verification documentation:

- **[Test Results Summary](test-specs/test-results-summary.md)**
- **[Caching Tests](test-specs/test-set-01-caching.md)**
- **[Search Tests](test-specs/test-set-02-search.md)**
- **[Interpreter Tests](test-specs/test-set-03-interpreter.md)**
- **[Preview Tests](test-specs/test-set-04-preview.md)**

## Getting Help

- **GitHub Issues**: [Report bugs or request features](https://github.com/7rikazhexde/gist-cache-rs/issues)
- **Documentation**: You're already here!
- **Command Help**: Run `gist-cache-rs --help` for CLI reference

## License

This project is licensed under the MIT License.
