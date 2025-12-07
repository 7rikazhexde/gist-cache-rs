# Test Strategy and Execution Guide

## Overview

gist-cache-rs's test strategy is structured in a three-layer architecture: unit tests, integration tests, and E2E tests.

**Current Coverage**: 68.95% (533/773 lines)
**Number of Automated Tests**: 153 (Unit 120 + Integration 33)
**Manual E2E Tests**: 26 cases

---

## Test Pyramid Structure

| Test Type | Count | Location | Execution Method |
| :---------- | :---- | :------------------------ | :--------------------- |
| **Unit Tests** | 120 | `src/` within `#[cfg(test)]` | `cargo test` (auto) |
| **Integration Tests** | 33 | `tests/` directory | `cargo test` (auto) |
| **E2E Tests** | 26 cases | `docs/tests/` | Manual execution |
| **Total** | **153** | - | - |

**Principles of the Test Pyramid**:

- Most unit tests (78%) - Fast, no external dependencies
- Integration tests in the middle (22%) - Verifies actual process execution
- Minimal E2E tests (manual) - Comprehensive user-centric verification

---

## Test Execution

### Basic Test Execution

```bash
# Run all tests
cargo test

# With verbose output
cargo test -- --nocapture

# Run specific tests only
cargo test test_cache_content
```

### Running Tests with `ignore` Attribute

```bash
# Run tests including those with the ignore attribute
cargo test -- --include-ignored

# Run only tests with the ignore attribute
cargo test -- --ignored
```

### Coverage Measurement

```bash
# Display coverage to standard output
cargo tarpaulin --out Stdout

# Generate HTML report
cargo tarpaulin --out Html --output-dir coverage

# See docs/testing/COVERAGE.md for details
```

---

## Test Configuration

### 1. Unit Tests (120)

**Location**: `src/` within `#[cfg(test)]` module

**Coverage Target**:

- Data structures and serialization (`cache/types.rs`)
- Cache management logic (`cache/content.rs`, `cache/update.rs`)
- Search logic (`search/query.rs`)
- CLI argument processing (`cli.rs`)
- Configuration management (`config.rs`)
- Error handling (`error.rs`)
- Basic functionality of the execution runner (`execution/runner.rs`)
- GitHub API mock (`github/client.rs`)

**Features**:

- Fast execution (no external dependencies)
- Excludes GitHub API dependencies with MockGitHubClient
- Automatable in CI/CD

### 2. Integration Tests (33)

**Location**: `tests/` directory

#### 2.1 CLI Tests (`tests/cli_tests.rs`) - 15

- Verification of command-line argument processing
- Subcommand operation verification (`update`, `run`, `cache`)
- Error case verification (authentication errors, no cache, etc.)
- Flag combination verification (`--preview`, `--force`, `--filename`, etc.)

#### 2.2 Interpreter Integration Tests (`tests/integration_test.rs`) - 12

- Bash, Python, Node.js execution tests
- TypeScript (ts-node, deno, bun) execution tests
- Ruby, Perl, PHP execution tests
- Argument passing, error handling
- Preview mode operation verification

#### 2.3 Runner Tests (`tests/runner_test.rs`) - 6

- Detailed verification of script execution logic
- Cache creation operation verification
- Download mode operation verification
- Force file-based execution verification
- Multi-file Gist selection logic

**Features**:

- Verifies actual process execution
- Unix environment only (`#[cfg_attr]` controlled)
- Automatically skipped if interpreter is not installed

### 3. E2E Tests (26 cases, manual)

**Location**: `docs/tests/`

**Test Sets**:

1. Caching functionality (`test_set_01_caching.md`) - 8 cases
2. Search functionality (`test_set_02_search.md`) - 6 cases
3. Interpreter (`test_set_03_interpreter.md`) - 7 cases
4. Preview functionality (`test_set_04_preview.md`) - 5 cases

**Features**:

- Comprehensive verification using actual Gists
- User-centric workflow verification
- Detailed reproducible steps included

---

## Testing Policy

### What to Cover with Unit Tests

✅ **Targets**:

- Business logic
- Data transformation/serialization
- Error handling
- Mockable external dependencies

❌ **Not Targets**:

- External process execution (bash, python, etc.) → Verified by integration tests
- GitHub CLI (`gh` command) → Replaced by MockGitHubClient, or #[ignore] tests
- User input processing → Verified by E2E tests

### Test Quality Metrics

**Target Coverage**: 60-70% (Standard for CLI tools)
**Current Coverage**: 68.95% ✅ Target achieved

**Reasons for Achievement**:

- Core logic has high coverage (types 100%, config 96%, content 83%, cli 78%)
- External process dependent code (runner.rs 20%, api.rs 8%) verified by integration tests
- Low coverage for thin wrappers is acceptable

---

## Troubleshooting

### Tests are filtered out

**Cause**: Execution in non-Unix environment, or interpreter not installed

**Solution**:

- Unix environment recommended for integration tests
- Install interpreters (bash, python, node, etc.)
- Or, automatic skipping is normal behavior

### Coverage cannot be measured

**Cause**: tarpaulin is not installed

**Solution**:

```bash
cargo install cargo-tarpaulin
```

### Integration tests fail

**Cause**: Interpreter (bash, python, node, etc.) not installed

**Solution**:

- Install necessary interpreters
- Or, skipping is normal behavior

---

## Detailed Documentation

- **Coverage Measurement**: [COVERAGE.md](./COVERAGE.md) - Measurement methods and module-specific details
- **Test Inventory**: [TEST_INVENTORY.md](./TEST_INVENTORY.md) - Classification and overview of all tests
- **GitHub CLI Related Test Evaluation**: [GH_TESTING_EVALUATION.md](./GH_TESTING_EVALUATION.md) - Evaluation of gh command tests

---

## References

- [Rust Testing Guide](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [cargo-tarpaulin](https://github.com/xd009642/tarpaulin)
- [mockall](https://docs.rs/mockall/latest/mockall/)

---

**Last Updated**: 2025-11-06
**Current Coverage**: 68.95%
**Number of Automated Tests**: 153
**Covered Lines**: 533/773 lines